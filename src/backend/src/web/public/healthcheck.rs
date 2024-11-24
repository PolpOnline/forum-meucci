use axum::response::IntoResponse;
use http::StatusCode;

#[utoipa::path(
    method(get),
    path = "/healthcheck",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
pub async fn healthcheck() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}
