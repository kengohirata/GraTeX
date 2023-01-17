use crate::parse::{self, Paragraph};
use anyhow::Result;
use arboard::Clipboard;
use std::fs::read_to_string;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opts {
    #[structopt(subcommand)]
    input: InputType,
    #[structopt(short)]
    output: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
pub enum InputType {
    #[structopt(name = "-f")]
    File { path: PathBuf },
    #[structopt(name = "-r")]
    Raw { text: String },
    #[structopt(name = "-c")]
    Clipboard,
    #[structopt(name = "-s")]
    StdIO,
}

pub fn run(opts: Opts) -> i32 {
    match run_result(opts) {
        Ok((paragraph, None)) => {
            println!("{paragraph}");
            0
        }
        Ok((paragraph, Some(out_path_buf))) => {
            let out_path = out_path_buf.as_path();
            match std::fs::File::options()
                .create(true)
                .append(true)
                .open(out_path)
            {
                Ok(mut file) => {
                    write!(file, "{paragraph}").expect("failed to write;");
                    0
                }
                Err(e) => {
                    eprintln!("failed to write to {}; {}", out_path.to_string_lossy(), e);
                    1
                }
            }
        }
        Err(err) => {
            eprintln!("failed to convert; {err}");
            1
        }
    }
}

fn run_result(opts: Opts) -> Result<(Paragraph, Option<PathBuf>)> {
    let raw_code = match opts.input {
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
    let mut paragraph = parse::Paragraph::from_str(&raw_code)
        .map_err(|err| anyhow::anyhow!("failed to parse; {}", err))?;
    paragraph.arrange();
    Ok((paragraph,opts.output))
}
