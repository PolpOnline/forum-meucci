use utoipa_axum::{router::OpenApiRouter, routes};

mod available_events;
mod me;
mod selected_events;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(me::me))
        .routes(routes!(available_events::available_events))
        .routes(routes!(selected_events::selected_events))
}
