use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{app::openapi::USER_TAG, models::user::UserType, users::AuthSession};

#[derive(Serialize, ToSchema)]
pub struct UserTypeResponse {
    #[schema(example = "normal")]
    user_type: UserType,
}

#[utoipa::path(
    get,
    path = "/user_type",
    responses(
        (status = OK, description = "Returns the user's type", body = UserTypeResponse),
        (status = UNAUTHORIZED, description = "Not logged in")
    ),
    security(
        ("session" = [])
    ),
    tag = USER_TAG,
)]
pub(super) async fn user_type(auth_session: AuthSession) -> impl IntoResponse {
    match auth_session.user {
        Some(user) => Json(UserTypeResponse {
            user_type: user.r#type,
        })
        .into_response(),
        None => StatusCode::UNAUTHORIZED.into_response(),
    }
}
