pub mod events;
mod rounds;

use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(events::events))
        .routes(routes!(rounds::rounds))
}
