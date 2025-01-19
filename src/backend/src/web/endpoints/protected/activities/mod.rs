use axum::middleware;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::middleware::end_bookings_routing::end_bookings_routing;

mod available;
mod selected;
mod set;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(set::set))
        .routes(routes!(available::available))
        .layer(middleware::from_fn(end_bookings_routing))
        .routes(routes!(selected::selected))
}
