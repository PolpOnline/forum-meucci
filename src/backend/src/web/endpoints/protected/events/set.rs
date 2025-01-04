use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Deserialize;
use tracing::info;
use utoipa::ToSchema;

use crate::{app::openapi::EVENTS_TAG, users::AuthSession};

#[derive(Deserialize, ToSchema)]
pub struct SetEventRequest {
    /// The id of the event to set to, do not provide to set absent on that
    /// round
    #[schema(example = 1, minimum = 1)]
    event_id: Option<i32>,
    /// The round to set the event to
    #[schema(example = 0, minimum = 0)]
    round: i32,
}

#[utoipa::path(
    patch,
    path = "/set",
    request_body = SetEventRequest,
    responses(
        (status = OK, description = "The event was set successfully"),
        (status = UNAUTHORIZED, description = "Not logged in"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("session" = [])
    ),
    tag = EVENTS_TAG,
)]
pub async fn set(auth_session: AuthSession, Json(req): Json<SetEventRequest>) -> impl IntoResponse {
    let user_id = match auth_session.user {
        Some(user) => user.id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let event_id = match req.event_id {
        Some(event_id) => event_id,
        None => {
            // Query the absent event
            match sqlx::query!(
                // language=PostgreSQL
                r#"
                    SELECT id FROM event WHERE name = 'absent'
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
        // language=PostgreSQL
        r#"
            INSERT INTO event_user (event_id, user_id, round)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, round) DO UPDATE SET event_id = EXCLUDED.event_id;
        "#,
        event_id,
        user_id,
        req.round,
    )
    .execute(&auth_session.backend.db)
    .await
    {
        Ok(_) => {}
        Err(e) => {
            info!("Failed to set event: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    StatusCode::OK.into_response()
}
