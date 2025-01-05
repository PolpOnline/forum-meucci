mod events;
mod presences;
mod rounds;
mod set_presence;
pub mod shared;

use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(events::events))
        .routes(routes!(rounds::rounds))
        .routes(routes!(presences::presences))
        .routes(routes!(set_presence::set_presence))
}
