use axum::response::IntoResponse;
use http::StatusCode;

use crate::app::openapi::SYSTEM_TAG;

#[utoipa::path(
    method(get),
    path = "/healthcheck",
    summary = "Healthcheck",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain", example = "OK"),
    ),
    tag = SYSTEM_TAG
)]
pub async fn healthcheck() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}
