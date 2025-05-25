use seeding::{
    record::{RecordProgressDistribution, RecordStatusDistribution},
    Pointercrate, Seeder, SeedingOptions,
};
use sqlx::postgres::PgPoolOptions;

mod gen;
mod seeding;

#[tokio::main]
async fn main() {
    // Read our .env and enable logging
    // Most logs are displayed at the info level, so add "RUST_LOG=info" to
    // your environment variables or .env file
    dotenv::dotenv().unwrap();
    env_logger::init();

    // Pool database connections for our seeder
    let pool = PgPoolOptions::default()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    // Connect to our pointercrate instance (all there really is right now is the connection pool)
    let instance = Pointercrate::new(pool);

    // Configure the seeding results
    let mut seeder = Seeder::new(
        instance,
        SeedingOptions {
            players: 2500,

            records: 100000,
            record_status_distribution: RecordStatusDistribution {
                approved: 35,
                rejected: 50, // 50% chance a record will be rejected
                under_consideration: 10,
                submitted: 5,
            },
            record_progress_distribution: RecordProgressDistribution {
                complete: 98, // 98% chance a record will be 100%
                incomplete: 2,
            },

            demons: 400,
            creators_per_demon: 1..5, // Each demon will have anywhere from 1 to 4 creators

            submitters: 90000,
        },
    );

    //// Nation pool is necessary for generating players
    // Gather the nationalities which will be assigned to newly generated players, if any.
    seeder.populate_nation_pool().await;

    //// Player pool is necessary for generating records and demons
    // Generate the specified amount of players, which are pooled and randomly assigned as
    // creators to levels, or may be given list records.
    seeder.populate_player_pool().await;
    // Gather existing players from the database and add them to the player pool. Every other
    // `_database` pool populating method retrieves existing items from the database
    // seeder.populate_player_pool_database().await;

    //// Demon pool is necessary for generating records
    // Generate and pool the specified quantity of demons
    seeder.populate_demon_pool().await;
    //seeder.populate_demon_pool_database().await;

    //// Submitter pool is necessary for generating records
    // Generate and pool the specified number of submitters
    seeder.populate_submitter_pool().await;
    //seeder.populate_submitter_pool_database().await;

    //// Record pool is necessary for NOTHING
    // Generate and pool the specified number of records
    // The record statuses are distributed roughly to what the `record_status_distribution` field
    // specifies. Same goes for 100% and non-100% records with `record_progress_distribution`
    seeder.populate_record_pool().await;
    //seeder.populate_record_pool_database().await;

    // Update the scores. Only necessary if any new records are added
    seeder.instance.update_scores().await;
}
