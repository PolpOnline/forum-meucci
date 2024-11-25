use axum::response::IntoResponse;
use http::StatusCode;
use crate::app::SYSTEM_TAG;

#[utoipa::path(
    method(get),
    path = "/healthcheck",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    ),
    tag = SYSTEM_TAG
)]
pub async fn healthcheck() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}
