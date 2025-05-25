use std::ops::Range;

use pointercrate_demonlist::player::recompute_scores;
use rand::rngs::ThreadRng;
use record::{RecordProgressDistribution, RecordStatusDistribution};
use sqlx::{pool::PoolConnection, Pool, Postgres};

pub mod demon;
pub mod nationality;
pub mod player;
pub mod record;
pub mod seeder;
pub mod submitter;
pub mod user;

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
    pub fn validate(&self) {
        if self.records < self.players {
            panic!("records cannot be less than players");
        }

        if self.submitters > self.records {
            panic!("submitters cannot be greater than records");
        }

        self.record_status_distribution.validate();
        self.record_progress_distribution.validate();
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
    pub fn new(instance: Pointercrate, options: SeedingOptions) -> Seeder {
        options.validate();

        Seeder {
            instance,
            options,

            player_pool: Vec::new(),
            demon_pool: Vec::new(),
            record_pool: Vec::new(),
            submitter_pool: Vec::new(),
            nation_pool: Vec::new(),
        }
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

    pub async fn connect(&self) -> PoolConnection<Postgres> {
        self.pool.acquire().await.unwrap()
    }

    pub async fn update_scores(&self) {
        let mut connection = self.connect().await;

        recompute_scores(&mut connection).await.unwrap();

        log::info!("Recomputed scores");
    }
}
