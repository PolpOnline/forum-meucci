use std::sync::LazyLock;

use color_eyre::Result;
use dotenvy::dotenv;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use web::App;

pub mod app;
pub mod auth;
pub mod fixtures;
pub mod middleware;
pub mod models;
pub mod users;
pub mod web;

pub static PRODUCTION: LazyLock<bool> = LazyLock::new(|| std::env::var("PRODUCTION").is_ok());
pub static SITE_URL: LazyLock<String> =
    LazyLock::new(|| std::env::var("SITE_URL").unwrap_or_else(|_| "http://localhost:5173".into()));
pub static BACKEND_URL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://localhost:3000".into())
});
pub static EMAIL_DOMAIN: LazyLock<String> =
    LazyLock::new(|| std::env::var("EMAIL_DOMAIN").unwrap());

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
            |_| {
                "axum_login=info,tower_sessions=info,sqlx=warn,tower_http=debug,forum_meucci=debug"
                    .into()
            },
        )))
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    dotenv().unwrap_or_default();

    if *PRODUCTION {
        info!("System: Production mode");
    } else {
        info!("System: Development mode");
    }

    #[cfg(debug_assertions)]
    {
        use app::cli::{Args, Command};
        use clap::Parser;

        let args = Args::parse();

        let app = App::new().await?;

        match args.command {
            None => app.serve().await,
            Some(Command::SeedUser) => fixtures::user::seed(app.db).await,
            Some(Command::SeedActivity) => fixtures::activity::seed(app.db).await,
            Some(Command::SeedAll) => {
                fixtures::user::seed(app.db.clone()).await?;
                fixtures::activity::seed(app.db).await
            }
        }
    }

    // Run the app in production without the CLI
    #[cfg(not(debug_assertions))]
    {
        App::new().await?.serve().await
    }
}
