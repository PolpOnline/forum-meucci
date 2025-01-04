mod me;
mod my_type;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(me::me))
        .routes(routes!(my_type::my_type))
}
