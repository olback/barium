use fern::{
    Dispatch,
    InitError,
    colors::{
        Color,
        ColoredLevelConfig
    }
};
use crate::is_debug;

fn get_level() -> log::LevelFilter {

    match is_debug!() {
        true => log::LevelFilter::Trace,
        false => log::LevelFilter::Info
    }

}

pub fn configure() -> Result<(), InitError> {

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
        .level(get_level())
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log")?)
        .apply()?;

    Ok(())

}