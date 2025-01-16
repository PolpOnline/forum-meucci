use utoipa_axum::router::OpenApiRouter;

mod activities;
mod admin;
mod user;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/activities", activities::router())
        .nest("/user", user::router())
        .nest("/admin", admin::router())
}
