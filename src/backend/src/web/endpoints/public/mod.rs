use utoipa_axum::router::OpenApiRouter;

mod info;
pub mod system;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .merge(system::router())
        .merge(info::router())
}
