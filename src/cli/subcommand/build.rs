use std::fs::read_to_string;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opts {
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
}

pub fn run(opts: Opts) -> i32 {
    todo!()
}