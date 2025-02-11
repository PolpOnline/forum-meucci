use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    app::openapi::ADMIN_TAG, models::user::UserType, users::AuthSession,
    web::endpoints::protected::admin::shared::user_has_access_to_activity,
};

#[derive(Deserialize, ToSchema)]
pub struct CallRegisterRequest {
    /// The ID of the activity
    #[schema(example = 1)]
    activity_id: i32,
    /// The round number
    #[schema(example = 1)]
    round: i32,
}

#[utoipa::path(
    patch,
    path = "/call_register",
    summary = "Call Register",
    request_body = CallRegisterRequest,
    responses(
        (status = OK, description = "Last edited by updated"),
        (status = UNAUTHORIZED, description = "Not logged in"),
        (status = FORBIDDEN, description = "Not an admin or host"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error"),
        (status = 425, description = "Registrations have not started yet"),
    ),
    security(
        ("session" = [])
    ),
    tag = ADMIN_TAG,
)]
pub async fn call_register(
    auth_session: AuthSession,
    Json(req): Json<CallRegisterRequest>,
) -> impl IntoResponse {
    let (user_type, user_id) = match auth_session.user {
        Some(ref user) => (user.r#type, user.id),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if user_type == UserType::Normal {
        return StatusCode::FORBIDDEN.into_response();
    }

    match user_has_access_to_activity(&auth_session, user_type, user_id, req.activity_id).await {
        Ok(true) => {}
        Ok(false) => return StatusCode::FORBIDDEN.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }

    match sqlx::query!(
        r#"
        UPDATE activity_user
        SET last_edited_by = $1
        WHERE activity_id = $2
          AND round = $3
        "#,
        user_id,
        req.activity_id,
        req.round,
    )
    .execute(&auth_session.backend.db)
    .await
    {
        Ok(_) => {}
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    StatusCode::OK.into_response()
}
