use std::ops::Range;

use pointercrate_demonlist::{
    creator::Creator,
    demon::{Demon, MinimalDemon},
    error::DemonlistError,
    player::DatabasePlayer,
};
use rand::{seq::SliceRandom, Rng};
use sqlx::{PgConnection, Row};

use crate::{error::CliError, gen::name::generate_demon_name};

use super::Pointercrate;

impl Pointercrate {
    pub async fn register_demon(
        &mut self,
        position: &i32,
        player_pool: &Vec<i32>,
        creators_per_demon: &Range<i32>,
    ) -> Result<i32, CliError> {
        let mut connection = self.connect().await?;

        let verifier = &self.random_player(&player_pool, &mut connection).await?;
        let publisher = &self.random_player(&player_pool, &mut connection).await?;

        let demon = sqlx::query(r#"INSERT INTO demons (name, position, requirement, verifier, publisher) VALUES ($1, $2, $3, $4, $5) RETURNING id"#)
            .bind(&generate_demon_name(&mut self.rng))
            .bind(position)
            .bind(&self.rng.gen_range(1..100))
            .bind(&verifier.base.id)
            .bind(&publisher.base.id)
            .fetch_one(&mut *connection)
            .await?
            .get("id");

        let demon = MinimalDemon::by_id(demon, &mut connection).await?;

        let num_creators = self.rng.gen_range(creators_per_demon.to_owned());

        for _ in 0..num_creators {
            match Creator::insert(
                &demon,
                &DatabasePlayer::by_id(
                    self.random_player(&player_pool, &mut connection)
                        .await?
                        .base
                        .id,
                    &mut connection,
                )
                .await?,
                &mut connection,
            )
            .await
            {
                Ok(_) => Ok(()),
                Err(DemonlistError::CreatorExists) => {
                    log::warn!(
                    "Randomly retrieved duplicate creator for this demon, creator insertion failed"
                );
                    Ok(())
                }
                Err(err) => Err(err),
            }?;
        }

        log::info!(
            "Registered demon with ID {} and {} creators",
            &demon.id,
            &num_creators
        );

        Ok(demon.id)
    }

    pub async fn random_demon(
        &mut self,
        demon_pool: &Vec<i32>,
        connection: &mut PgConnection,
    ) -> Result<Demon, CliError> {
        Ok(Demon::by_id(
            demon_pool.choose(&mut self.rng).unwrap().to_owned(),
            connection,
        )
        .await?)
    }

    pub async fn position_offset(&self) -> Result<i32, CliError> {
        let mut connection = self.connect().await?;

        let max: i16 = sqlx::query_scalar(r#"SELECT MAX(position) FROM demons"#)
            .fetch_one(&mut *connection)
            .await
            .unwrap_or(0);

        Ok((max + 1) as i32)
    }
}
