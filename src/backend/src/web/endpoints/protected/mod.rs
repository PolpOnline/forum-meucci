use axum::middleware;
use utoipa_axum::router::OpenApiRouter;

use crate::middleware::start_date_routing::start_date_routing;

mod activities;
mod admin;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/activities", activities::router())
        .nest("/admin", admin::router())
        .layer(middleware::from_fn(start_date_routing))
}
