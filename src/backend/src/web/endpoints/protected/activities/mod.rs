use utoipa_axum::{router::OpenApiRouter, routes};

mod available;
mod bookings_start_date;
mod selected;
mod set;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(available::available))
        .routes(routes!(selected::selected))
        .routes(routes!(set::set))
        .routes(routes!(bookings_start_date::bookings_start_date))
}
