use structopt::StructOpt;

pub mod convert;
pub use convert::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "gratex")]
pub struct Opts {
    #[structopt(flatten)]
    pub verbose: structopt_flags::Verbose,

    #[structopt(flatten)]
    pub command: convert::Opts,
}
