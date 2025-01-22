use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::info;

use crate::app::App;

impl App {
    pub(super) async fn setup_db() -> color_eyre::Result<PgPool> {
        info!("SQLx: Connecting to the database...");

        let database_url = match std::env::var("DATABASE_PRIVATE_URL") {
            Ok(url) => {
                info!("SQLx: Using DATABASE_PRIVATE_URL");
                url
            }
            Err(_) => {
                info!("SQLx: Using DATABASE_URL");
                std::env::var("DATABASE_URL")?
            }
        };

        let pool = PgPoolOptions::new()
            .max_connections(50)
            .connect(&database_url)
            .await?;

        info!("SQLx: Connected to the database");

        sqlx::migrate!().run(&pool).await?;

        info!("SQLx: Migrations run");

        Ok(pool)
    }
}
