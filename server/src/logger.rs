use fern::{
    Dispatch,
    InitError,
    colors::{
        Color,
        ColoredLevelConfig
    }
};
use crate::is_debug;

fn get_log_level(log_level: Option<log::LevelFilter>) -> log::LevelFilter {

    match log_level {
        Some(ll) => ll,
        None => {
            eprintln!("Log level not set. Using default log level for current environment.");
            match is_debug!() {
                true => {
                    eprintln!("Environment is debug, defaulting to TRACE log level.");
                    log::LevelFilter::Trace
                },
                false => {
                    eprintln!("Environment is release, defaulting to INFO log level.");
                    log::LevelFilter::Info
                }
            }
        }
    }

}

pub fn configure(log_level: Option<log::LevelFilter>) -> Result<(), InitError> {

    let colors = ColoredLevelConfig::new()
        .debug(Color::BrightMagenta)
        .info(Color::BrightGreen)
        .warn(Color::Yellow)
        .error(Color::Red);

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}#{}] [{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                record.line().unwrap_or(0),
                colors.color(record.level()),
                message
            ))
        })
        .level(get_log_level(log_level))
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log")?)
        .apply()?;

    Ok(())

}
