use utoipa_axum::router::OpenApiRouter;

mod admin;
mod events;
mod user;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/events", events::router())
        .nest("/user", user::router())
        .nest("/admin", admin::router())
}
