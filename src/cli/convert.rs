use crate::arrange;
use crate::preprocess::preprocess;

use super::super::ast;

use super::super::token;
use anyhow::Context;
use anyhow::Result;
use arboard::Clipboard;
use std::env;
use std::fs::{self, read_to_string};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opts {
    #[structopt(subcommand)]
    input: InputType,

    /// Output file
    #[structopt(short)]
    output: Option<PathBuf>,

    /// Launch a Chrome editor page (via Grammarly) with the converted text
    #[structopt(long)]
    grammarly: bool,

    /// File to back the Grammarly editing session (defaults to a temporary file)
    #[structopt(long = "grammarly-path", parse(from_os_str), value_name = "FILE")]
    grammarly_path: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
pub enum InputType {
    /// input from a File
    #[structopt(name = "-f")]
    File { path: PathBuf },
    /// input from Cli
    #[structopt(name = "-r")]
    Raw { text: String },
    /// input from your Clipboard
    #[structopt(name = "-c")]
    Clipboard,
    /// input from StdIO
    #[structopt(name = "-s")]
    StdIO,
}

pub fn run(opts: Opts) -> i32 {
    if opts.grammarly_path.is_some() && !opts.grammarly {
        eprintln!("--grammarly-path can only be used together with --grammarly");
        return 1;
    }

    let Opts {
        input,
        output,
        grammarly,
        grammarly_path,
    } = opts;

    match convert_input(input) {
        Ok(converted) => {
            if let Some(out_path_buf) = output {
                let out_path = out_path_buf.as_path();
                match std::fs::File::options()
                    .create(true)
                    .append(true)
                    .open(out_path)
                {
                    Ok(mut file) => {
                        if let Err(err) = write!(file, "{converted}") {
                            eprintln!("failed to write to {}; {}", out_path.to_string_lossy(), err);
                            return 1;
                        }
                    }
                    Err(e) => {
                        eprintln!("failed to write to {}; {}", out_path.to_string_lossy(), e);
                        return 1;
                    }
                }
            } else if !grammarly {
                println!("{converted}");
            }

            if grammarly {
                match launch_grammarly_session(&converted, grammarly_path) {
                    Ok(target_path) => {
                        eprintln!(
                            "Opened Chrome editor for {} (press Ctrl+C here once you are done)",
                            target_path.display()
                        );
                        0
                    }
                    Err(err) => {
                        eprintln!("failed to launch Grammarly helper; {err}");
                        1
                    }
                }
            } else {
                0
            }
        }
        Err(err) => {
            eprintln!("failed to convert; {err}");
            1
        }
    }
}

pub fn convert_input(input: InputType) -> Result<String> {
    let mut raw_code = match input {
        InputType::File { path } => read_to_string(&path)
            .map_err(|err| anyhow::anyhow!("failed to load {}; {}", path.to_string_lossy(), err))?,
        InputType::Raw { text } => text,
        InputType::Clipboard => Clipboard::new()?.get_text()?,
        InputType::StdIO => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };
    preprocess(&mut raw_code);
    let token = token::Document::from_str(&raw_code)
        .map_err(|err| anyhow::anyhow!("failed to parse; {}", err))?;
    let ast = ast::token_to_ast(token);

    let mut ast_str = format!("{ast}");
    arrange::arrange_text_string(&mut ast_str);
    Ok(ast_str)
}

const OPEN_CHROME_SCRIPT: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/open-chrome-to-edit"));

fn launch_grammarly_session(content: &str, requested_path: Option<PathBuf>) -> Result<PathBuf> {
    let target_path = match requested_path {
        Some(path) => path,
        None => default_grammarly_path()?,
    };

    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory for {}", target_path.display()))?;
    }

    fs::write(&target_path, content).with_context(|| {
        format!(
            "failed to write converted text to {}",
            target_path.display()
        )
    })?;

    let script_path = resolve_helper_script()?;

    if let Err(status) = run_with_uv(&script_path, &target_path) {
        if let LaunchError::Command(err) = status {
            return Err(err);
        }

        run_with_python(&script_path, &target_path)?;
    }

    Ok(target_path)
}

fn default_grammarly_path() -> Result<PathBuf> {
    let base = env::temp_dir().join("gratex");
    fs::create_dir_all(&base).with_context(|| format!("failed to create {}", base.display()))?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_secs();

    Ok(base.join(format!("grammarly-{timestamp}.txt")))
}

fn resolve_helper_script() -> Result<PathBuf> {
    if let Ok(path) = env::var("GRATEX_OPEN_CHROME_SCRIPT") {
        let candidate = PathBuf::from(path);
        if candidate.exists() {
            return Ok(candidate);
        }
        anyhow::bail!(
            "helper script overridden via GRATEX_OPEN_CHROME_SCRIPT but file was not found"
        );
    }

    let manifest_script = Path::new(env!("CARGO_MANIFEST_DIR")).join("open-chrome-to-edit");
    if manifest_script.exists() {
        return Ok(manifest_script);
    }

    let cache_dir = env::temp_dir().join("gratex");
    fs::create_dir_all(&cache_dir)
        .with_context(|| format!("failed to create {}", cache_dir.display()))?;
    let cached_path = cache_dir.join("open-chrome-to-edit.py");

    let needs_write = fs::read_to_string(&cached_path)
        .map(|current| current != OPEN_CHROME_SCRIPT)
        .unwrap_or(true);
    if needs_write {
        fs::write(&cached_path, OPEN_CHROME_SCRIPT)
            .with_context(|| format!("failed to write {}", cached_path.display()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&cached_path)
                .with_context(|| format!("failed to read metadata for {}", cached_path.display()))?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&cached_path, perms).with_context(|| {
                format!("failed to set permissions for {}", cached_path.display())
            })?;
        }
    }

    Ok(cached_path)
}

enum LaunchError {
    NotAvailable,
    Command(anyhow::Error),
}

fn run_with_uv(script_path: &Path, target_path: &Path) -> Result<(), LaunchError> {
    match Command::new("uv")
        .arg("run")
        .arg(script_path)
        .arg(target_path)
        .status()
    {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(LaunchError::Command(anyhow::anyhow!(
            "uv exited with status {}",
            exit_status_as_string(status)
        ))),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Err(LaunchError::NotAvailable),
        Err(err) => Err(LaunchError::Command(
            anyhow::Error::new(err).context("failed to invoke uv to launch Grammarly helper"),
        )),
    }
}

fn run_with_python(script_path: &Path, target_path: &Path) -> Result<()> {
    let mut attempts = Vec::new();
    for candidate in ["python3", "python"] {
        match Command::new(candidate)
            .arg(script_path)
            .arg(target_path)
            .status()
        {
            Ok(status) if status.success() => return Ok(()),
            Ok(status) => {
                attempts.push(format!(
                    "{candidate} exited with status {}",
                    exit_status_as_string(status)
                ));
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
            Err(err) => {
                return Err(anyhow::Error::new(err)
                    .context(format!("failed to invoke {candidate} for Grammarly helper")));
            }
        }
    }

    if attempts.is_empty() {
        anyhow::bail!("could not find a Python interpreter (tried python3, python)");
    }

    anyhow::bail!("{}", attempts.join("; "));
}

fn exit_status_as_string(status: std::process::ExitStatus) -> String {
    match status.code() {
        Some(code) => code.to_string(),
        None => "terminated by signal".to_owned(),
    }
}
