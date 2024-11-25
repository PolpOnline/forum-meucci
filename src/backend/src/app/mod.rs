use std::str::FromStr;

use axum::middleware;
use axum::routing::get;
use axum_login::{
    tower_sessions::{Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use http::StatusCode;
use openidconnect::core::CoreClient;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::{
    compression::CompressionLayer, decompression::DecompressionLayer, trace::TraceLayer,
};
use tower_sessions::cookie::{Key, SameSite};
use tower_sessions_redis_store::{
    fred::prelude::{ClientLike, RedisConfig as RedisFredConfig, RedisPool as RedisFredPool},
    RedisStore,
};
use tower_sessions_redis_store::fred::types::ReconnectPolicy;
use tracing::info;
use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};
use crate::{
    auth::google_oauth::build_google_oauth_client,
    custom_login_required,
    middleware::set_cache_control::set_cache_control,
    users::LoginBackend,
    web::{auth, protected, public},
    BACKEND_URL,
};

pub const AUTH_TAG: &str = "Auth";
pub const SYSTEM_TAG: &str = "System";
pub const USER_TAG: &str = "User";

#[derive(OpenApi)]
#[openapi(
    modifiers(&ApiDocSecurityAddon),
    tags(
        (name = AUTH_TAG, description = "Endpoints to authenticate users"),
        (name = SYSTEM_TAG, description = "Endpoints to monitor the system"),
        (name = USER_TAG, description = "Endpoints related to users")
    )
)]
struct ApiDoc;

struct ApiDocSecurityAddon;

impl Modify for ApiDocSecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "session",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("meucci_forum_id"))),
            )
        }
    }
}

pub struct App {
    db: PgPool,
    redis_fred: RedisFredPool,
    google_oauth_client: CoreClient,
}

impl App {
    pub async fn new() -> color_eyre::Result<Self> {
        let db = Self::setup_db().await?;
        let redis_fred = Self::setup_redis_fred().await?;

        let backend_url = BACKEND_URL.clone();
        let redirect_uri = format!("{}/auth/callback", backend_url);

        let google_oauth_client = build_google_oauth_client(redirect_uri).await;

        Ok(Self {
            db,
            redis_fred,
            google_oauth_client,
        })
    }

    pub async fn serve(self) -> color_eyre::Result<()> {
        // Session layer.
        //
        // This uses `tower-sessions` to establish a layer that will provide the session
        // as a request extension.
        let session_store = RedisStore::new(self.redis_fred.clone());

        // Generate a cryptographic key to sign the session cookie.
        let key = &std::env::var("COOKIE_KEY")?;
        let key = parse_cookie_key(key);

        let session_layer = SessionManagerLayer::new(session_store)
            .with_name("meucci_forum_id")
            .with_domain("localhost")
            .with_secure(std::env::var("COOKIE_DOMAIN").is_ok())
            .with_same_site(SameSite::Lax)
            .with_expiry(Expiry::OnInactivity(
                tower_sessions::cookie::time::Duration::days(7),
            ))
            .with_signed(key);

        // Auth service.
        //
        // This combines the session layer with our backendOld to establish the auth
        // service which will provide the auth session as a request extension.
        let backend = LoginBackend::new(self.db, self.google_oauth_client);
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
            .merge(protected::router())
            .route_layer(custom_login_required!(
                LoginBackend,
                (StatusCode::UNAUTHORIZED, "You are not logged in.")
            ))
            .nest("/auth", auth::router())
            .merge(public::router())
            .layer(auth_layer)
            .layer(middleware::from_fn(set_cache_control))
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new())
            .layer(DecompressionLayer::new())
            .split_for_parts();
        
        let router = {
            let api_json = serde_json::to_value(api.clone()).expect("Failed to convert api to JSON");

            router
                .route("/openapi.json", get(move || async { axum::Json(api_json) }))
                .merge(Scalar::with_url("/scalar", api))
        };

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

        info!("Axum: Listening on {}", listener.local_addr()?);

        // Ensure we use a shutdown signal to abort the deletion task.
        axum::serve(listener, router.into_make_service()).await?;

        Ok(())
    }

    async fn setup_db() -> color_eyre::Result<PgPool> {
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
            .max_connections(5)
            .connect(&database_url)
            .await?;

        info!("SQLx: Connected to the database");

        sqlx::migrate!().run(&pool).await?;

        info!("SQLx: Migrations run");

        Ok(pool)
    }

    async fn setup_redis_fred() -> color_eyre::Result<RedisFredPool> {
        info!("Redis Fred: Connecting to Redis (to manage sessions)...");

        let db_num = 0u8;

        let redis_url = std::env::var("REDIS_URL")?;
        let redis_url = format!("{}/{}", redis_url, db_num);

        let config = RedisFredConfig::from_url(&redis_url)?;

        let pool = RedisFredPool::new(config, None, None, Some(ReconnectPolicy::default()), 6)?;

        pool.init().await?;

        info!("Redis Fred: Connected to Redis (to manage sessions)");

        Ok(pool)
    }
}

fn parse_cookie_key(cookie_key: &str) -> Key {
    let key: Vec<u8> = cookie_key[1..cookie_key.len() - 1]
        .split(", ")
        .filter_map(|byte| u8::from_str(byte.trim()).ok())
        .collect();

    Key::from(&key)
}
