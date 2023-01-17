use gratex::cli;
///
use log::Level;
use structopt::StructOpt;
use structopt_flags::LogLevel;

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

fn main() {
    let opts: cli::Opts = cli::Opts::from_args();

    if let Some(level) = opts.verbose.get_log_level() {
        setup_logger(level).unwrap();
    }

    let exit_code = cli::convert::run(opts.command);

    std::process::exit(exit_code)
}
