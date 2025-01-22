pub mod cli;
pub mod config;
mod db;
pub mod openapi;
mod redis;
mod unredirectable_async_http_client;

use std::str::FromStr;

use axum::{middleware, routing::get};
use axum_login::{
    tower_sessions::{Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use http::StatusCode;
use sqlx::PgPool;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    decompression::{DecompressionLayer, RequestDecompressionLayer},
    trace::TraceLayer,
};
use tower_sessions::cookie::{Key, SameSite};
use tower_sessions_redis_store::{fred::prelude::Pool as FredPool, RedisStore};
use tracing::info;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use crate::{
    app::{config::Config, openapi::ApiDoc},
    auth::google_oauth::{build_google_oauth_client, CoreClient},
    custom_login_required,
    middleware::set_cache_control::set_cache_control,
    users::LoginBackend,
    web::endpoints::{auth, protected, public},
    BACKEND_URL,
};

pub struct App {
    pub db: PgPool,
    redis_fred: FredPool,
    unredirectable_async_http_client: reqwest::Client,
    google_oauth_client: CoreClient,
    config: Config,
}

impl App {
    pub async fn new() -> color_eyre::Result<Self> {
        let backend_url = BACKEND_URL.clone();
        let redirect_uri = format!("{}/auth/callback", backend_url);

        let unredirectable_async_http_client = Self::get_unredirectable_async_client()?;

        let (db, redis_fred, google_oauth_client) = tokio::try_join!(
            Self::setup_db(),
            Self::setup_redis_fred(),
            build_google_oauth_client(redirect_uri, &unredirectable_async_http_client)
        )?;

        Ok(Self {
            db,
            redis_fred,
            unredirectable_async_http_client,
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
        let backend = LoginBackend::new(
            // Clone the pool to shut it down later.
            self.db.clone(),
            self.unredirectable_async_http_client,
            self.google_oauth_client,
            self.config,
        );
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
            .merge(protected::router())
            .route_layer(custom_login_required!(
                LoginBackend,
                (StatusCode::UNAUTHORIZED, "You are not logged in.")
            ))
            .nest("/auth", auth::router())
            .merge(public::router())
            .layer(
                ServiceBuilder::new()
                    .layer(auth_layer)
                    .layer(middleware::from_fn(set_cache_control))
                    .layer(TraceLayer::new_for_http()),
            )
            .split_for_parts();

        let router = {
            let api_json =
                serde_json::to_value(api.clone()).expect("Failed to convert api to JSON");

            router
                .route("/openapi.json", get(move || async { axum::Json(api_json) }))
                .merge(Scalar::with_url("/scalar", api))
        };

        let router = router.layer(
            ServiceBuilder::new()
                .layer(DecompressionLayer::new())
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new()),
        );

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

        info!("Axum: Listening on {}", listener.local_addr()?);

        axum::serve(listener, router.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await?;

        let pool_close_fut = self.db.close();

        futures::future::join_all(vec![pool_close_fut]).await;

        Ok(())
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutting down...");
}

fn parse_cookie_key(cookie_key: &str) -> Key {
    let key: Vec<u8> = cookie_key[1..cookie_key.len() - 1]
        .split(", ")
        .filter_map(|byte| u8::from_str(byte.trim()).ok())
        .collect();

    Key::from(&key)
}
