mod bookings_start_date;

use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(bookings_start_date::bookings_start_date))
}
