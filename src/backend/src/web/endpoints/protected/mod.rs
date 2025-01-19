use axum::middleware;
use utoipa_axum::router::OpenApiRouter;

use crate::middleware::booking_start_date::booking_start_date;

mod activities;
mod admin;
mod user;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/activities", activities::router())
        .layer(middleware::from_fn(booking_start_date))
        .nest("/user", user::router())
        .nest("/admin", admin::router())
}
