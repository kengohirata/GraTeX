/// 
use log::Level;
use structopt_flags::LogLevel;
use std::io;
use GraTeX::cli;
use structopt::StructOpt;

fn setup_logger(level: Level) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level.to_level_filter())
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main () {
    let opts: cli::Opts = cli::Opts::from_args();

    if let Some(level) = opts.verbose.get_log_level() {
        setup_logger(level).unwrap();
    }

    let exit_code = match opts.sub_command {
        cli::SubCommand::Build(sub_opts) => cli::subcommand::build::run(sub_opts),
        cli::SubCommand::Completion(sub_opts) => cli::subcommand::completion::run(sub_opts),
    };

    std::process::exit(exit_code)
}