use pointercrate_demonlist::record::RecordStatus;
use rand::{distributions::WeightedIndex, prelude::Distribution, rngs::ThreadRng, Rng};
use serde::Deserialize;

use crate::error::CliError;

use super::Pointercrate;

#[derive(Debug, Deserialize)]
pub struct RecordStatusDistribution {
    pub approved: u8,
    pub rejected: u8,
    pub under_consideration: u8,
    pub submitted: u8,
}

impl RecordStatusDistribution {
    pub fn validate(&self) {
        if self.approved + self.rejected + self.under_consideration + self.submitted != 100 {
            panic!("sum of values in RecordStatusDistribution must be 100");
        }
    }

    pub fn sample(&self, rng: &mut ThreadRng) -> RecordStatus {
        match WeightedIndex::new(vec![
            self.approved,
            self.rejected,
            self.under_consideration,
            self.submitted,
        ])
        .unwrap()
        .sample(rng)
        {
            0 => RecordStatus::Approved,
            1 => RecordStatus::Rejected,
            2 => RecordStatus::UnderConsideration,
            _ => RecordStatus::Submitted,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RecordProgressDistribution {
    pub complete: u8,
    pub incomplete: u8,
}

pub enum RecordProgressDistributionType {
    Complete,
    Incomplete,
}

impl RecordProgressDistribution {
    pub fn validate(&self) {
        if self.complete + self.incomplete != 100 {
            panic!("sum of values in RecordProgressDistribution must be 100");
        }
    }

    pub fn sample(&self, rng: &mut ThreadRng) -> RecordProgressDistributionType {
        match WeightedIndex::new(vec![self.complete, self.incomplete])
            .unwrap()
            .sample(rng)
        {
            0 => RecordProgressDistributionType::Complete,
            _ => RecordProgressDistributionType::Incomplete,
        }
    }
}

impl Pointercrate {
    pub async fn register_record(
        &mut self,
        player_pool: &Vec<i32>,
        demon_pool: &Vec<i32>,
        submitter_pool: &Vec<i32>,
        status_distribution: &RecordStatusDistribution,
        progress_distribution: &RecordProgressDistribution,
    ) -> Result<i32, CliError> {
        let mut connection = self.connect().await?;

        let demon = self.random_demon(&demon_pool, &mut connection).await?;
        let player = self.random_player(&player_pool, &mut connection).await?;
        let submitter = self
            .random_submitter(&submitter_pool, &mut connection)
            .await?;

        let progress = match progress_distribution.sample(&mut self.rng) {
            RecordProgressDistributionType::Complete => 100,
            RecordProgressDistributionType::Incomplete => {
                self.rng.gen_range(demon.requirement..100)
            }
        };

        let status = status_distribution.sample(&mut self.rng);

        let record: i32 = sqlx::query_scalar(r#"INSERT INTO records (progress, status_, player, submitter, demon) VALUES ($1, CAST($2::TEXT as record_status), $3, $4, $5) RETURNING id"#)
            .bind(progress)
            .bind(status.to_sql().to_string())
            .bind(player.base.id)
            .bind(submitter.id)
            .bind(demon.base.id)
            .fetch_one(&mut *connection)
            .await?;

        log::info!("Registered record with ID {}", record);

        Ok(record)
    }
}
