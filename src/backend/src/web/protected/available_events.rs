use axum::{extract::Query, response::IntoResponse, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{app::openapi::EVENT_TAG, users::AuthSession};

#[derive(Deserialize, IntoParams)]
pub struct AvailableEventRequest {
    round: i32,
}

#[derive(Serialize, ToSchema)]
pub struct AvailableEventResponse {
    events: Vec<AvailableEventItem>,
}

#[derive(Serialize, ToSchema)]
pub struct AvailableEventItem {
    id: i32,
    name: String,
    description: String,
    room: String,
    available_seats: Option<i64>,
    total_seats: i64,
}

#[utoipa::path(
    get,
    path = "/available_events",
    params(AvailableEventRequest),
    responses(
        (status = OK, description = "Returns the available events", body = AvailableEventResponse),
        (status = UNAUTHORIZED, description = "Not logged in"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("session" = [])
    ),
    tag = EVENT_TAG,
)]
pub(super) async fn available_events(
    auth_session: AuthSession,
    Query(req): Query<AvailableEventRequest>,
) -> impl IntoResponse {
    let user_section = match auth_session.user {
        Some(user) => user.section,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let events = match sqlx::query_as!(
        AvailableEventItem,
        // language=PostgreSQL
        r#"
        SELECT
        event.id, event.name, event.description, event.room,
        (round_max_users.max_users - COUNT(event_user.user_id)) AS available_seats,
        round_max_users.max_users AS total_seats
        FROM
            event
        JOIN
            round_max_users ON event.id = round_max_users.event_id
        LEFT JOIN
            event_user ON event.id = event_user.event_id AND round_max_users.round = event_user.round
        WHERE
            round_max_users.round = $1 AND $2 >= event.minimum_section
        GROUP BY
            event.id, round_max_users.max_users
        HAVING
            (round_max_users.max_users - COUNT(event_user.user_id)) > 0
        "#,
        req.round,
        user_section,
    ).fetch_all(&auth_session.backend.db).await {
        Ok(r) => {r}
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Json(AvailableEventResponse { events }).into_response()
}
