use pointercrate_demonlist::nationality::Nationality;

use super::Seeder;

impl Seeder {
    #[allow(dead_code)]
    pub async fn populate_player_pool(&mut self) {
        log::info!("Populating player pool");

        for _ in 0..self.options.players {
            self.player_pool
                .push(self.instance.register_player(&self.nation_pool).await);
        }
    }

    #[allow(dead_code)]
    pub async fn populate_player_pool_database(&mut self) {
        log::info!("Populating player pool from database");

        let mut connection = self.instance.connect().await;

        let players: Vec<i32> = sqlx::query_scalar(r#"SELECT id FROM players"#)
            .fetch_all(&mut *connection)
            .await
            .unwrap();

        players.iter().for_each(|id| self.player_pool.push(*id));

        log::info!("Loaded {} players from database", players.len());
    }

    #[allow(dead_code)]
    pub async fn populate_demon_pool(&mut self) {
        log::info!("Populating demon pool");

        let offset = self.instance.position_offset().await;

        for position in offset..(self.options.demons + offset) {
            self.demon_pool.push(
                self.instance
                    .register_demon(
                        &(position),
                        &self.player_pool,
                        &self.options.creators_per_demon,
                    )
                    .await,
            );
        }
    }

    #[allow(dead_code)]
    pub async fn populate_demon_pool_database(&mut self) {
        log::info!("Populating demon pool from database");

        let mut connection = self.instance.connect().await;

        let demons: Vec<i32> = sqlx::query_scalar("SELECT id FROM demons")
            .fetch_all(&mut *connection)
            .await
            .unwrap();

        demons.iter().for_each(|id| self.demon_pool.push(*id));

        log::info!("Loaded {} demons from database", demons.len());
    }

    #[allow(dead_code)]
    pub async fn populate_record_pool(&mut self) {
        log::info!("Populating records pool");

        for _ in 0..self.options.records {
            self.record_pool.push(
                self.instance
                    .register_record(
                        &self.player_pool,
                        &self.demon_pool,
                        &self.submitter_pool,
                        &self.options.record_status_distribution,
                        &self.options.record_progress_distribution,
                    )
                    .await,
            );
        }
    }

    #[allow(dead_code)]
    pub async fn populate_record_pool_database(&mut self) {
        log::info!("Populating records pool from database");

        let mut connection = self.instance.connect().await;

        let records: Vec<i32> = sqlx::query_scalar("SELECT id FROM records")
            .fetch_all(&mut *connection)
            .await
            .unwrap();

        records.iter().for_each(|id| self.record_pool.push(*id));

        log::info!("Loaded {} records from database", records.len());
    }

    #[allow(dead_code)]
    pub async fn populate_submitter_pool(&mut self) {
        log::info!("Populating submitter pool");

        for _ in 0..self.options.submitters {
            self.submitter_pool
                .push(self.instance.register_submitter().await);
        }
    }

    #[allow(dead_code)]
    pub async fn populate_submitter_pool_database(&mut self) {
        log::info!("Populating submitter pool from database");

        let mut connection = self.instance.connect().await;

        let submitters: Vec<i32> = sqlx::query_scalar("SELECT submitter_id FROM submitters")
            .fetch_all(&mut *connection)
            .await
            .unwrap();

        submitters
            .iter()
            .for_each(|id| self.submitter_pool.push(*id));

        log::info!("Loaded {} submitters from database", submitters.len());
    }

    #[allow(dead_code)]
    pub async fn populate_nation_pool(&mut self) {
        log::info!("Populating nation pool from database");

        let mut connection = self.instance.connect().await;

        let nations = Nationality::all(&mut connection).await.unwrap();

        nations
            .iter()
            .for_each(|n| self.nation_pool.push(n.iso_country_code.clone()));
    }
}
