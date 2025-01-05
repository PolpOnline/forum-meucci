use axum::{extract::Path, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    app::{config::Config, openapi::ADMIN_TAG},
    models::user::UserType,
    users::AuthSession,
    web::schemas::event::{round_to_date, RoundConversionError},
};

#[derive(Deserialize, IntoParams)]
pub struct AdminRoundRequest {
    /// The ID of the event
    #[param(example = 1)]
    event_id: i32,
}

#[derive(Serialize, ToSchema)]
pub struct AdminRoundResponse {
    #[schema(example = "Event 1")]
    name: String,
    #[schema(example = "Room 1")]
    room: String,
    rounds: Vec<AdminRound>,
}

#[derive(Serialize, ToSchema)]
pub struct AdminRound {
    #[schema(example = 0, minimum = 0)]
    round: i32,
    date: DateTime<Utc>,
    #[schema(example = 10)]
    available_seats: Option<i64>,
    #[schema(example = 20)]
    total_seats: i64,
}

impl AdminRound {
    fn from_without_date(
        event: AdminRoundWithoutDate,
        config: &Config,
    ) -> Result<Self, RoundConversionError> {
        Ok(AdminRound {
            round: event.round,
            date: round_to_date(config, event.round)?,
            available_seats: event.available_seats,
            total_seats: event.total_seats,
        })
    }
}

pub struct AdminRoundWithoutDate {
    round: i32,
    available_seats: Option<i64>,
    total_seats: i64,
}

#[utoipa::path(
    get,
    path = "/rounds/{event_id}",
    params(AdminRoundRequest),
    responses(
        (status = OK, description = "List of the rounds for an event the user has access to", body = AdminRoundResponse),
        (status = UNAUTHORIZED, description = "Not logged in"),
        (status = FORBIDDEN, description = "Not an admin or host"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("session" = [])
    ),
    tag = ADMIN_TAG,
)]
pub async fn rounds(
    auth_session: AuthSession,
    Path(req): Path<AdminRoundRequest>,
) -> impl IntoResponse {
    let (user_type, user_id) = match auth_session.user {
        Some(ref user) => (user.r#type, user.id),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if user_type == UserType::Normal {
        return StatusCode::FORBIDDEN.into_response();
    }

    let event = match user_type {
        UserType::Admin => admin_event(&auth_session, req.event_id).await,
        UserType::Host => host_event(&auth_session, user_id, req.event_id).await,
        UserType::Normal => unreachable!(),
    };

    let event = match event {
        Ok(Some(r)) => r,
        // User does not have access to the event
        Ok(None) => return StatusCode::FORBIDDEN.into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let rounds = match sqlx::query_as!(
        AdminRoundWithoutDate,
        // language=PostgreSQL
        r#"
        SELECT round_max_users.round,
              (round_max_users.max_users - COUNT(event_user.user_id)) AS available_seats,
               round_max_users.max_users                              AS total_seats
        FROM round_max_users
            LEFT JOIN event_user
                ON round_max_users.event_id = event_user.event_id
                    AND round_max_users.round = event_user.round
        WHERE round_max_users.event_id = $1
        GROUP BY round_max_users.round, round_max_users.max_users;
        "#,
        req.event_id
    )
    .fetch_all(&auth_session.backend.db)
    .await
    {
        Ok(r) => r,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let rounds: Result<Vec<_>, _> = rounds
        .into_iter()
        .map(|event| AdminRound::from_without_date(event, &auth_session.backend.config))
        .collect();

    let rounds = match rounds {
        Ok(r) => r,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    Json(AdminRoundResponse {
        name: event.name,
        room: event.room,
        rounds,
    })
    .into_response()
}

struct EventInterrogation {
    name: String,
    room: String,
}

/// Get all events for an admin user (all events)
async fn admin_event(
    auth_session: &AuthSession,
    event_id: i32,
) -> Result<Option<EventInterrogation>, sqlx::Error> {
    // Intentionally allow access to events that should not be displayed
    let name = sqlx::query_as!(
        EventInterrogation,
        // language=PostgreSQL
        r#"
        SELECT name, room
        FROM event
        WHERE event.id = $1
        "#,
        event_id
    )
    .fetch_one(&auth_session.backend.db)
    .await?;

    Ok(Some(name))
}

/// Get all events for a host user (only events they are hosting, checking the
/// event_admin table)
async fn host_event(
    auth_session: &AuthSession,
    user_id: i32,
    event_id: i32,
) -> Result<Option<EventInterrogation>, sqlx::Error> {
    // Disallow access to events that should not be displayed
    // Get the name of the event while checking if the user has access to it
    let name = sqlx::query_as!(
        EventInterrogation,
        // language=PostgreSQL
        r#"
        SELECT name, room
        FROM event
        JOIN event_admin ON event.id = event_admin.event_id
        WHERE event_admin.user_id = $1 AND event.id = $2 AND event.should_display IS TRUE
        "#,
        user_id,
        event_id
    )
    .fetch_optional(&auth_session.backend.db)
    .await?;

    Ok(name)
}
