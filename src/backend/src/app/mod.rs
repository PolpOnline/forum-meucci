pub mod config;
mod db;
pub mod openapi;
mod redis;

use std::str::FromStr;

use axum::{middleware, routing::get};
use axum_login::{
    tower_sessions::{Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use http::StatusCode;
use openidconnect::core::CoreClient;
use sqlx::PgPool;
use tower_http::{
    compression::CompressionLayer, decompression::DecompressionLayer, trace::TraceLayer,
};
use tower_sessions::cookie::{Key, SameSite};
use tower_sessions_redis_store::{fred::prelude::Pool as FredPool, RedisStore};
use tracing::info;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use crate::{
    app::{config::Config, openapi::ApiDoc},
    auth::google_oauth::build_google_oauth_client,
    custom_login_required,
    middleware::set_cache_control::set_cache_control,
    users::LoginBackend,
    web::endpoints::{auth, protected, public},
    BACKEND_URL,
};

pub struct App {
    db: PgPool,
    redis_fred: FredPool,
    google_oauth_client: CoreClient,
    config: Config,
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
            config: Config::init(),
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
            .with_secure(false)
            .with_domain(std::env::var("COOKIE_DOMAIN")?)
            .with_same_site(SameSite::Lax)
            .with_expiry(Expiry::OnInactivity(
                tower_sessions::cookie::time::Duration::days(7),
            ))
            .with_signed(key);

        #[cfg(not(debug_assertions))]
        let session_layer = session_layer.with_secure(true);

        // Auth service.
        //
        // This combines the session layer with our backendOld to establish the auth
        // service which will provide the auth session as a request extension.
        let backend = LoginBackend::new(self.db, self.google_oauth_client, self.config);
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
            let api_json =
                serde_json::to_value(api.clone()).expect("Failed to convert api to JSON");

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
}

fn parse_cookie_key(cookie_key: &str) -> Key {
    let key: Vec<u8> = cookie_key[1..cookie_key.len() - 1]
        .split(", ")
        .filter_map(|byte| u8::from_str(byte.trim()).ok())
        .collect();

    Key::from(&key)
}
