mod registrations_start_date;

use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(registrations_start_date::registrations_start_date))
}
