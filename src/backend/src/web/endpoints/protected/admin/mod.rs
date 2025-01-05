mod events;
mod presences;
mod rounds;
pub mod shared;

use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(events::events))
        .routes(routes!(rounds::rounds))
        .routes(routes!(presences::presences))
}
