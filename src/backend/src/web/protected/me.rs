use axum::Json;
use axum::response::IntoResponse;
use http::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;
use crate::{app::USER_TAG, users::AuthSession};

#[derive(Serialize, ToSchema)]
pub struct User {
    name: Option<String>,
    email: String,
}

#[utoipa::path(
    get,
    path = "/me",
    responses(
        (status = OK, description = "Returns the user's info", body = User),
        (status = UNAUTHORIZED, description = "Not logged in")
    ),
    security(
        ("session" = [])
    ),
    tag = USER_TAG,

)]
pub(super) async fn me(auth_session: AuthSession) -> impl IntoResponse {
    if let Some(user) = auth_session.user {
        return Json(User {
            name: user.name,
            email: user.email,
        }).into_response();
    }

    StatusCode::UNAUTHORIZED.into_response()
}
