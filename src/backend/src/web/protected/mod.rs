use utoipa_axum::{router::OpenApiRouter, routes};

mod available_events;
mod me;
mod selected_events;
mod set_event;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(me::me))
        .routes(routes!(available_events::available_events))
        .routes(routes!(selected_events::selected_events))
        .routes(routes!(set_event::set_event))
}
