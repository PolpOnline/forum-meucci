use axum::{extract::Path, response::IntoResponse};
use axum_serde::Sonic;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{app::openapi::ACTIVITIES_TAG, users::AuthSession};

#[derive(Deserialize, IntoParams)]
pub struct AvailableActivityRequest {
    /// The round of the activity (0-indexed)
    #[param(example = 0, minimum = 0)]
    round: i32,
}

#[derive(Serialize, ToSchema)]
pub struct AvailableActivityResponse {
    activities: Vec<AvailableActivity>,
}

#[derive(Serialize, ToSchema)]
pub struct AvailableActivity {
    #[schema(example = 1)]
    id: i32,
    #[schema(example = "Activity 1")]
    name: String,
    #[schema(example = "This is the description of the first activity")]
    description: String,
    #[schema(example = "Room 1")]
    room: String,
    #[schema(example = 10)]
    used_seats: Option<i64>,
    #[schema(example = 20)]
    total_seats: i64,
}

#[utoipa::path(
    get,
    path = "/available/{round}",
    summary = "Available Activities",
    params(AvailableActivityRequest),
    responses(
        (status = OK, description = "Returns the available activities", body = AvailableActivityResponse),
        (status = UNAUTHORIZED, description = "Not logged in"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error"),
        (status = 425, description = "Registrations have not started yet"),
        (status = GONE, description = "Registrations have ended"),
    ),
    security(
        ("session" = [])
    ),
    tag = ACTIVITIES_TAG,
)]
pub async fn available(
    auth_session: AuthSession,
    Path(req): Path<AvailableActivityRequest>,
) -> impl IntoResponse {
    let user_class = match auth_session.user {
        Some(user) => user.class,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let activities = match sqlx::query_as!(
        AvailableActivity,
        r#"
        SELECT forum_activity.id,
               forum_activity.name,
               forum_activity.description,
               forum_activity.room,
               COUNT(forum_activity_user.user_id) AS used_seats,
               forum_round_max_users.max_users AS total_seats
        FROM forum_activity
                 JOIN
             forum_round_max_users ON forum_activity.id = forum_round_max_users.activity_id
                 LEFT JOIN
             forum_activity_user ON forum_activity.id = forum_activity_user.activity_id AND forum_round_max_users.round = forum_activity_user.round
        WHERE forum_activity.should_display IS TRUE
          AND forum_round_max_users.round = $1
          AND $2 >= forum_activity.minimum_class
        GROUP BY forum_activity.id, forum_activity.name, forum_activity.description, forum_activity.room, forum_round_max_users.max_users
        HAVING (forum_round_max_users.max_users - COUNT(forum_activity_user.user_id)) > 0
        ORDER BY LOWER(forum_activity.name)
        "#,
        req.round,
        user_class,
    ).fetch_all(&auth_session.backend.db).await {
        Ok(r) => r,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Sonic(AvailableActivityResponse { activities }).into_response()
}
