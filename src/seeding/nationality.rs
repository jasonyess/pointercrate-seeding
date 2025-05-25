use pointercrate_demonlist::nationality::Nationality;
use rand::seq::SliceRandom;
use sqlx::PgConnection;

use super::Pointercrate;

impl Pointercrate {
    pub async fn random_nation(
        &mut self,
        nation_pool: &Vec<String>,
        connection: &mut PgConnection,
    ) -> Nationality {
        Nationality::by_country_code_or_name(nation_pool.choose(&mut self.rng).unwrap(), connection)
            .await
            .unwrap()
    }
}
