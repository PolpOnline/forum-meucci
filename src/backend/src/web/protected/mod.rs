use utoipa_axum::router::OpenApiRouter;

mod events;
mod schemas;
mod user;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/events", events::router())
        .nest("/user", user::router())
}
