use crate::arrange;

use super::super::ast;

use super::super::token;
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

    /// Output file
    #[structopt(short)]
    output: Option<PathBuf>,
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
    match (run_result(opts.input), opts.output) {
        (Ok(ast), None) => {
            println!("{ast}");
            0
        }
        (Ok(ast), Some(out_path_buf)) => {
            let out_path = out_path_buf.as_path();
            match std::fs::File::options()
                .create(true)
                .append(true)
                .open(out_path)
            {
                Ok(mut file) => {
                    write!(file, "{ast}").expect("failed to write;");
                    0
                }
                Err(e) => {
                    eprintln!("failed to write to {}; {}", out_path.to_string_lossy(), e);
                    1
                }
            }
        }
        (Err(err), _) => {
            eprintln!("failed to convert; {err}");
            1
        }
    }
}

fn run_result(opts: InputType) -> Result<String> {
    let raw_code = match opts {
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
    let token = token::Document::from_str(&raw_code)
        .map_err(|err| anyhow::anyhow!("failed to parse; {}", err))?;
    let ast = ast::token_to_ast(token);

    let mut ast_str = format!("{ast}");
    arrange::arrange_text_string(&mut ast_str);
    Ok(ast_str)
}
