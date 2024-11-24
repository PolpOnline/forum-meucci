mod healthcheck;
mod sys_info;

use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(sys_info::sys_info))
        .routes(routes!(healthcheck::healthcheck))
}
