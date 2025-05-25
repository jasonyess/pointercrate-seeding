use crate::gen::name::generate_player_name;
use pointercrate_demonlist::player::{DatabasePlayer, Player};
use rand::seq::SliceRandom;
use sqlx::PgConnection;

use super::Pointercrate;

impl Pointercrate {
    pub async fn register_player(&mut self, nation_pool: &Vec<String>) -> i32 {
        let mut connection = self.connect().await;

        let player = DatabasePlayer::by_name_or_create(
            &generate_player_name(&mut self.rng),
            &mut *connection,
        )
        .await
        .unwrap();

        let mut player = Player::by_id(player.id, &mut connection).await.unwrap();
        player
            .set_nationality(
                Some(self.random_nation(&nation_pool, &mut connection).await),
                &mut connection,
            )
            .await
            .unwrap();

        log::info!(
            "Registered player with ID {} and nationality {}",
            &player.base.id,
            &player.nationality.unwrap().iso_country_code,
        );

        player.base.id
    }

    pub async fn random_player(
        &mut self,
        player_pool: &Vec<i32>,
        connection: &mut PgConnection,
    ) -> Player {
        Player::by_id(
            player_pool.choose(&mut self.rng).unwrap().to_owned(),
            connection,
        )
        .await
        .unwrap()
    }
}
