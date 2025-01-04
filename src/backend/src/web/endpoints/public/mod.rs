use utoipa_axum::router::OpenApiRouter;

pub mod system;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().merge(system::router())
}
