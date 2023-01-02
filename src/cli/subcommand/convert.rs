use crate::parse::{self, Paragraph};
use anyhow::Result;
use std::fs::read_to_string;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opts {
    #[structopt(short)]
    file: Option<PathBuf>,
    #[structopt(short)]
    raw: Option<String>,
    #[structopt(short)]
    output: Option<PathBuf>,
}

pub fn run(opts: Opts) -> i32 {
    match run_result(opts) {
        Ok((paragraph, None)) => {
            println!("{}", paragraph);
            0
        }
        Ok((paragraph, Some(out_path_buf))) => {
            let out_path = out_path_buf.as_path();
            match std::fs::File::options().append(true).open(out_path) {
                Ok(mut file) => {
                    write!(file, "{}", paragraph).expect("failed to write;");
                    0
                }
                Err(e) => {
                    eprintln!("failed to write to {}; {}", out_path.to_string_lossy(), e);
                    1
                }
            }
        }
        Err(err) => {
            eprintln!("failed to convert; {}", err);
            1
        }
    }
}

fn run_result(opts: Opts) -> Result<(Paragraph, Option<PathBuf>)> {
    let raw_code = match (opts.file, opts.raw) {
        (None, None) => return Result::Err(anyhow::anyhow!("No file was given.")),
        (Some(_), Some(raw)) => {
            eprintln!("Give one code per execution. Converting the raw code.");
            raw
        }
        (None, Some(raw)) => raw,
        (Some(path), None) => read_to_string(&path)
            .map_err(|err| anyhow::anyhow!("failed to load {}; {}", path.to_string_lossy(), err))?,
    };
    parse::Paragraph::from_str(&raw_code)
        .map(|p| (p, opts.output))
        .map_err(|err| anyhow::anyhow!("failed to parse; {}", err))
}
