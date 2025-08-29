use axum::middleware;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::middleware::end_registrations_routing::end_registrations_routing;

mod available;
mod selected;
mod set;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(set::set))
        .routes(routes!(available::available))
        .layer(middleware::from_fn(end_registrations_routing))
        .routes(routes!(selected::selected))
}
