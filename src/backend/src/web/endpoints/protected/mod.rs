use utoipa_axum::router::OpenApiRouter;

mod forum;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().nest("/forum", forum::router())
}
