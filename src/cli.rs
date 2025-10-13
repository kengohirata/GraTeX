use structopt::StructOpt;

pub mod convert;
pub use convert::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "gratex")]
/// GraTeX: An easy converter of TeX files to readable text for a grammer checker.
pub struct Opts {
    #[structopt(flatten)]
    pub verbose: structopt_flags::Verbose,

    #[structopt(flatten)]
    pub command: convert::Opts,
}
