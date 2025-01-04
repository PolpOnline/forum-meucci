use utoipa_axum::{router::OpenApiRouter, routes};

mod me;
mod schemas;
mod user_type;
mod events;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/events", events::router())
        .routes(routes!(me::me))
        .routes(routes!(user_type::user_type))
}
