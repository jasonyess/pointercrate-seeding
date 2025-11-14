use std::{fs::read_to_string, path::PathBuf};

use serde::Deserialize;

use crate::{error::CliError, seeding::SeedingOptions};

#[derive(Debug, Deserialize)]
pub struct DatabaseOptions {
    pub max_connections: u32,
    pub database_url: String,
}

#[derive(Debug, Deserialize)]
pub struct CategoryOutputOptions {
    pub generate_new: bool,
    pub use_existing: bool,
}

#[derive(Debug, Deserialize)]
pub struct OutputOptions {
    pub players: CategoryOutputOptions,
    pub demons: CategoryOutputOptions,
    pub submitters: CategoryOutputOptions,
    pub records: CategoryOutputOptions,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub log_level: String,
    pub database_options: DatabaseOptions,
    pub seeding_options: SeedingOptions,
    pub output_options: OutputOptions,
}

impl ConfigFile {
    pub fn read(path: &PathBuf) -> Result<ConfigFile, CliError> {
        Ok(serde_json::from_str(&read_to_string(path)?)
            .map_err(|err| CliError::InvalidConfigFile(err))?)
    }
}
