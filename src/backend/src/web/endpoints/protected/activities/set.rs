use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Deserialize;
use tracing::info;
use utoipa::ToSchema;

use crate::{app::openapi::ACTIVITIES_TAG, users::AuthSession};

#[derive(Deserialize, ToSchema)]
pub struct SetActivityRequest {
    /// The id of the activity to set to, do not provide to set absent on that
    /// round
    #[schema(example = 1, minimum = 1)]
    activity_id: Option<i32>,
    /// The round to set the activity to
    #[schema(example = 0, minimum = 0)]
    round: i32,
}

#[utoipa::path(
    patch,
    path = "/set",
    summary = "Set Activity",
    request_body = SetActivityRequest,
    responses(
        (status = OK, description = "The activity was set successfully"),
        (status = UNAUTHORIZED, description = "Not logged in"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("session" = [])
    ),
    tag = ACTIVITIES_TAG,
)]
pub async fn set(
    auth_session: AuthSession,
    Json(req): Json<SetActivityRequest>,
) -> impl IntoResponse {
    let user_id = match auth_session.user {
        Some(user) => user.id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let activity_id = match req.activity_id {
        Some(activity_id) => activity_id,
        None => {
            // Query the absent activity
            match sqlx::query!(
                r#"
                    SELECT id FROM activity WHERE name = 'absent'
                "#,
            )
            .fetch_one(&auth_session.backend.db)
            .await
            {
                Ok(row) => row.id,
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
    };

    match sqlx::query!(
        r#"
            INSERT INTO activity_user (activity_id, user_id, round)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, round) DO UPDATE SET activity_id = EXCLUDED.activity_id;
        "#,
        activity_id,
        user_id,
        req.round,
    )
    .execute(&auth_session.backend.db)
    .await
    {
        Ok(_) => {}
        Err(e) => {
            info!("Failed to set activity: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    StatusCode::OK.into_response()
}
