use std::net::{IpAddr, Ipv4Addr};

use pointercrate_demonlist::submitter::Submitter;
use rand::{seq::SliceRandom, Rng};
use sqlx::PgConnection;

use super::Pointercrate;

impl Pointercrate {
    pub async fn register_submitter(&mut self) -> i32 {
        let mut connection = self.connect().await;

        let submitter = Submitter::create_submitter(
            IpAddr::V4(Ipv4Addr::new(
                self.rng.gen(),
                self.rng.gen(),
                self.rng.gen(),
                self.rng.gen(),
            )),
            &mut connection,
        )
        .await
        .unwrap();

        log::info!("Registered submitter with ID {}", submitter.id);

        submitter.id
    }

    pub async fn random_submitter(
        &mut self,
        submitter_pool: &Vec<i32>,
        connection: &mut PgConnection,
    ) -> Submitter {
        Submitter::by_id(
            submitter_pool.choose(&mut self.rng).unwrap().to_owned(),
            connection,
        )
        .await
        .unwrap()
    }
}
