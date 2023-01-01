use structopt::StructOpt;

pub mod subcommand;
use subcommand::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "gratex")]
pub struct Opts {
    #[structopt(flatten)]
    pub verbose: structopt_flags::Verbose,

    #[structopt(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    Convert(convert::Opts),
    Completion(completion::Opts),
}
