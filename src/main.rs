use std::path::PathBuf;

use seeding::{Pointercrate, Seeder};
use sqlx::postgres::PgPoolOptions;

use crate::{config::ConfigFile, error::CliError};

mod config;
mod error;
mod gen;
mod logger;
mod seeding;

async fn execute() -> Result<(), CliError> {
    let args: Vec<String> = std::env::args().collect();

    let path = PathBuf::from(args.get(1).ok_or(CliError::MissingArgument(1, "path"))?);
    let config = ConfigFile::read(&path)?;

    logger::setup_logger(&config.log_level)?;

    let pool = PgPoolOptions::default()
        .max_connections(config.database_options.max_connections)
        .connect(&config.database_options.database_url)
        .await?;

    log::info!("Connected to database");

    let instance = Pointercrate::new(pool);
    let mut seeder = Seeder::new(instance, config.seeding_options);

    log::info!("Initialized seeder");

    seeder.populate_nation_pool().await?;

    if config.output_options.players.use_existing {
        seeder.populate_player_pool_database().await?;
    }
    if config.output_options.demons.use_existing {
        seeder.populate_demon_pool_database().await?;
    }
    if config.output_options.submitters.use_existing {
        seeder.populate_submitter_pool_database().await?;
    }
    if config.output_options.records.use_existing {
        seeder.populate_record_pool_database().await?;
    }

    if config.output_options.players.generate_new {
        seeder.populate_player_pool().await?;
    }
    if config.output_options.demons.generate_new {
        seeder.populate_demon_pool().await?;
    }
    if config.output_options.submitters.generate_new {
        seeder.populate_submitter_pool().await?;
    }
    if config.output_options.records.generate_new {
        seeder.populate_record_pool().await?;
    }

    seeder.instance.update_scores().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    if let Err(e) = execute().await {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    Ok(())
}
