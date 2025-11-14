use fern::colors::{Color, ColoredLevelConfig};
use sqlx::types::chrono::Local;

use crate::error::CliError;

pub fn setup_logger(log_level: &str) -> Result<(), CliError> {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::Green);

    fern::Dispatch::new()
        .format(move |out, msg, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                Local::now().format("%H:%M:%S"),
                colors.color(record.level()),
                msg
            ));
        })
        .level(
            log_level
                .parse()
                .map_err(|err| CliError::InvalidLogLevel(err))?,
        )
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    Ok(())
}
