use std::sync::LazyLock;

use color_eyre::Result;
use dotenvy::dotenv;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use web::App;

pub mod app;
pub mod auth;
pub mod fixtures;
pub mod middleware;
pub mod models;
pub mod users;
pub mod web;

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

    let indicatif_layer = IndicatifLayer::new();

    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
            |_| {
                "axum_login=info,tower_sessions=info,sqlx=warn,tower_http=debug,forum_meucci=debug"
                    .into()
            },
        )))
        .with(tracing_subscriber::fmt::layer().with_writer(indicatif_layer.get_stderr_writer()))
        .with(indicatif_layer)
        .try_init()?;

    dotenv().unwrap_or_default();

    let app = App::new().await?;

    #[cfg(debug_assertions)]
    {
        use app::cli::{Args, Command};
        use clap::Parser;

        let args = Args::parse();

        match args.command {
            None => app.serve().await,
            Some(Command::SeedUser(args)) => fixtures::user::seed(&app.db, args.write).await,
            Some(Command::SeedForumActivity(args)) => {
                fixtures::forum_activity::seed(&app.db, args.write).await
            }
            Some(Command::SeedForumAdmin(args)) => {
                fixtures::forum_admin::seed(&app.db, args.write).await
            }
            Some(Command::SeedForumAll(args)) => {
                fixtures::user::seed(&app.db, args.write).await?;
                fixtures::forum_activity::seed(&app.db, args.write).await?;
                fixtures::forum_admin::seed(&app.db, args.write).await
            }
            Some(Command::SeedForumHosts(args)) => {
                fixtures::forum_activity_host::seed(&app.db, args.write).await
            }
            Some(Command::SortOutForumUsers(args)) => {
                fixtures::forum_sort_out::sort_out_users(&app.db, args.write).await
            }
            Some(Command::ExportForumRounds) => {
                fixtures::forum_export_rounds::export_rounds(&app.db, &app.config).await
            }
            Some(Command::ExportForumPresences) => {
                fixtures::forum_export_presences::export_presences(&app.db, &app.config).await
            }
        }
    }

    // Run the app in production without the CLI
    #[cfg(not(debug_assertions))]
    {
        app.serve().await
    }
}
