use utoipa_axum::{router::OpenApiRouter, routes};

mod me;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(me::me))
}
