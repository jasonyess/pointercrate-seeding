use std::ops::Range;

use pointercrate_demonlist::player::recompute_scores;
use rand::rngs::ThreadRng;
use record::{RecordProgressDistribution, RecordStatusDistribution};
use serde::Deserialize;
use sqlx::{pool::PoolConnection, Pool, Postgres};

use crate::error::CliError;

pub mod demon;
pub mod nationality;
pub mod player;
pub mod record;
pub mod seeder;
pub mod submitter;
pub mod user;

#[derive(Debug, Deserialize)]
pub struct SeedingOptions {
    pub players: i32,

    pub records: i32,
    pub record_status_distribution: RecordStatusDistribution,
    pub record_progress_distribution: RecordProgressDistribution,

    pub demons: i32,
    pub creators_per_demon: Range<i32>,

    pub submitters: i32,
}

impl SeedingOptions {
    pub fn validate(&self) -> Result<(), CliError> {
        if self.records < self.players {
            return Err(CliError::InvalidProportions("players", "records"));
        }

        if self.submitters > self.records {
            return Err(CliError::InvalidProportions("submitters", "records"));
        }

        self.record_status_distribution.validate()?;
        self.record_progress_distribution.validate()?;

        Ok(())
    }
}

pub struct Seeder {
    pub instance: Pointercrate,
    pub options: SeedingOptions,

    player_pool: Vec<i32>,
    demon_pool: Vec<i32>,
    record_pool: Vec<i32>,
    submitter_pool: Vec<i32>,
    nation_pool: Vec<String>,
}

impl Seeder {
    pub fn new(instance: Pointercrate, options: SeedingOptions) -> Result<Seeder, CliError> {
        options.validate()?;

        Ok(Seeder {
            instance,
            options,

            player_pool: Vec::new(),
            demon_pool: Vec::new(),
            record_pool: Vec::new(),
            submitter_pool: Vec::new(),
            nation_pool: Vec::new(),
        })
    }
}

pub struct Pointercrate {
    pool: Pool<Postgres>,
    rng: ThreadRng,
}

impl Pointercrate {
    pub fn new(pool: Pool<Postgres>) -> Pointercrate {
        Pointercrate {
            pool,
            rng: rand::thread_rng(),
        }
    }

    pub async fn connect(&self) -> Result<PoolConnection<Postgres>, CliError> {
        Ok(self.pool.acquire().await?)
    }

    pub async fn update_scores(&self) -> Result<(), CliError> {
        let mut connection = self.connect().await?;

        recompute_scores(&mut connection).await?;

        log::info!("Recomputed scores");

        Ok(())
    }
}
