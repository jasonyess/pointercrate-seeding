use std::io;

use derive_more::Display;
use pointercrate_core::error::{CoreError, PointercrateError};
use pointercrate_demonlist::error::DemonlistError;
use pointercrate_user::error::UserError;

#[derive(Debug, Display)]
pub enum CliError {
    #[display("Missing argument #{}: {}", _0, _1)]
    MissingArgument(u8, &'static str),
    #[display("Malformed config file\n{}", _0)]
    InvalidConfigFile(serde_json::Error),
    #[display("Invalid log level: {}", _0)]
    InvalidLogLevel(log::ParseLevelError),
    #[display("Attempted to use existing data from empty table '{}'", _0)]
    EmptyTable(&'static str),

    #[display("IO error\n{}", _0)]
    IoError(io::Error),
    #[display("Database error\n{}", _0)]
    DatabaseError(sqlx::Error),
    #[display("Pointercrate error\n{}", _0)]
    PointercrateError(u16),
}

impl From<io::Error> for CliError {
    fn from(value: io::Error) -> Self {
        CliError::IoError(value)
    }
}

impl From<sqlx::Error> for CliError {
    fn from(value: sqlx::Error) -> Self {
        CliError::DatabaseError(value)
    }
}

impl From<CoreError> for CliError {
    fn from(value: CoreError) -> Self {
        CliError::PointercrateError(value.status_code())
    }
}

impl From<DemonlistError> for CliError {
    fn from(value: DemonlistError) -> Self {
        CliError::PointercrateError(value.status_code())
    }
}

impl From<UserError> for CliError {
    fn from(value: UserError) -> Self {
        CliError::PointercrateError(value.status_code())
    }
}
