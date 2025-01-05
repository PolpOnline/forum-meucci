use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{app::openapi::ADMIN_TAG, models::user::UserType, users::AuthSession};

#[derive(Serialize, ToSchema)]
pub struct AdminEventResponse {
    events: Vec<AdminEvent>,
}

#[derive(Serialize, ToSchema)]
pub struct AdminEvent {
    #[schema(example = 1)]
    id: i32,
    #[schema(example = "Event 1")]
    name: String,
    #[schema(example = "This is the description of event 1")]
    description: String,
    #[schema(example = "Room 1")]
    room: String,
}

#[utoipa::path(
    get,
    path = "/events",
    summary = "Events List",
    responses(
        (status = OK, description = "List of the events the user has access to, if admin all events, if host only those they are hosting", body = AdminEventResponse),
        (status = UNAUTHORIZED, description = "Not logged in"),
        (status = FORBIDDEN, description = "Not an admin or host"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("session" = [])
    ),
    tag = ADMIN_TAG,
)]
pub(super) async fn events(auth_session: AuthSession) -> impl IntoResponse {
    let (user_type, user_id) = match auth_session.user {
        Some(ref user) => (user.r#type, user.id),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let events = match user_type {
        UserType::Admin => admin_events(&auth_session).await,
        UserType::Host => host_events(&auth_session, user_id).await,
        UserType::Normal => return StatusCode::FORBIDDEN.into_response(),
    };

    let events = match events {
        Ok(r) => r,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Json(AdminEventResponse { events }).into_response()
}

/// Get all events for an admin user (all events)
async fn admin_events(auth_session: &AuthSession) -> Result<Vec<AdminEvent>, sqlx::Error> {
    let events = sqlx::query_as!(
        AdminEvent,
        // language=PostgreSQL
        r#"
        SELECT event.id            AS id,
               event.name          AS name,
               event.description   AS description,
               event.room          AS room
        FROM event
        WHERE event.should_display IS TRUE
        ORDER BY event.name;
        "#
    )
    .fetch_all(&auth_session.backend.db)
    .await?;

    Ok(events)
}

/// Get all events for a host user (only events they are hosting, checking the
/// event_admin table)
async fn host_events(
    auth_session: &AuthSession,
    user_id: i32,
) -> Result<Vec<AdminEvent>, sqlx::Error> {
    let events = sqlx::query_as!(
        AdminEvent,
        // language=PostgreSQL
        r#"
        SELECT event.id            AS id,
               event.name          AS name,
               event.description   AS description,
               event.room          AS room
        FROM event
                 JOIN event_admin ON event.id = event_admin.event_id
        WHERE event.should_display IS TRUE AND event_admin.user_id = $1
        ORDER BY event.name;
        "#,
        user_id
    )
    .fetch_all(&auth_session.backend.db)
    .await?;

    Ok(events)
}
