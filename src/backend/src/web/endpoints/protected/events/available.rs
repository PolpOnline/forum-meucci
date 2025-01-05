use axum::{extract::Path, response::IntoResponse, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{app::openapi::EVENTS_TAG, users::AuthSession};

#[derive(Deserialize, IntoParams)]
pub struct AvailableEventRequest {
    /// The round of the event (0-indexed)
    #[param(example = 0, minimum = 0)]
    round: i32,
}

#[derive(Serialize, ToSchema)]
pub struct AvailableEventResponse {
    events: Vec<AvailableEvent>,
}

#[derive(Serialize, ToSchema)]
pub struct AvailableEvent {
    #[schema(example = 1)]
    id: i32,
    #[schema(example = "Event 1")]
    name: String,
    #[schema(example = "This is the description of event 1")]
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
    summary = "Available Events",
    params(AvailableEventRequest),
    responses(
        (status = OK, description = "Returns the available events", body = AvailableEventResponse),
        (status = UNAUTHORIZED, description = "Not logged in"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("session" = [])
    ),
    tag = EVENTS_TAG,
)]
pub async fn available(
    auth_session: AuthSession,
    Path(req): Path<AvailableEventRequest>,
) -> impl IntoResponse {
    let user_section = match auth_session.user {
        Some(user) => user.section,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let events = match sqlx::query_as!(
        AvailableEvent,
        // language=PostgreSQL
        r#"
        SELECT event.id,
               event.name,
               event.description,
               event.room,
               COUNT(event_user.user_id) AS used_seats,
               round_max_users.max_users AS total_seats
        FROM event
                 JOIN
             round_max_users ON event.id = round_max_users.event_id
                 LEFT JOIN
             event_user ON event.id = event_user.event_id AND round_max_users.round = event_user.round
        WHERE event.should_display IS TRUE
          AND round_max_users.round = $1
          AND $2 >= event.minimum_section
        GROUP BY event.id, event.name, event.description, event.room, round_max_users.max_users
        HAVING (round_max_users.max_users - COUNT(event_user.user_id)) > 0
        ORDER BY LOWER(event.name)
        "#,
        req.round,
        user_section,
    ).fetch_all(&auth_session.backend.db).await {
        Ok(r) => r,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Json(AvailableEventResponse { events }).into_response()
}
