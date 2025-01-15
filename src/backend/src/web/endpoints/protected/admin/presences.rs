use axum::{extract::Path, response::IntoResponse, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    app::openapi::ADMIN_TAG, models::user::UserType, users::AuthSession,
    web::endpoints::protected::admin::shared::get_event,
};

#[derive(Deserialize, IntoParams)]
pub struct AdminPresenceRequest {
    /// The ID of the event
    #[param(example = 1)]
    event_id: i32,
    /// The round number
    #[param(example = 1)]
    round: i32,
}

#[derive(Serialize, ToSchema)]
pub struct AdminPresenceResponse {
    #[schema(example = "Event 1")]
    name: String,
    #[schema(example = "Room 1")]
    room: String,
    #[schema(example = 20)]
    total_seats: i32,
    presences: Vec<Presence>,
}

#[derive(Serialize, ToSchema)]
pub struct Presence {
    #[schema(example = 1, minimum = 1)]
    id: i32,
    #[schema(example = "John Doe")]
    name: String,
    #[schema(example = "A")]
    section: Option<String>,
    #[schema(example = "1")]
    class: i32,
    #[schema(example = false)]
    present: bool,
}

#[utoipa::path(
    get,
    path = "/presences/{event_id}/{round}",
    summary = "Presences List",
    params(AdminPresenceRequest),
    responses(
        (status = OK, description = "List of the presences for a given event and round", body = AdminPresenceResponse),
        (status = UNAUTHORIZED, description = "Not logged in"),
        (status = FORBIDDEN, description = "Not an admin or host"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("session" = [])
    ),
    tag = ADMIN_TAG,
)]
pub async fn presences(
    auth_session: AuthSession,
    Path(req): Path<AdminPresenceRequest>,
) -> impl IntoResponse {
    let (user_type, user_id) = match auth_session.user {
        Some(ref user) => (user.r#type, user.id),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if user_type == UserType::Normal {
        return StatusCode::FORBIDDEN.into_response();
    }

    let event = get_event(&auth_session, user_type, user_id, req.event_id).await;

    let event = match event {
        Ok(Some(r)) => r,
        // User does not have access to the event
        Ok(None) => return StatusCode::FORBIDDEN.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let presences_fut = sqlx::query_as!(
        Presence,
        // language=PostgreSQL
        r#"
        SELECT "user".id,
               "user".section,
               "user".class,
               COALESCE("user".name, "user".email) AS "name!: String",
               event_user.joined_at IS NOT NULL    AS "present!: bool"
        FROM event_user
                 JOIN "user" ON event_user.user_id = "user".id
        WHERE event_user.event_id = $1
          AND event_user.round = $2
        ORDER BY name;
        "#,
        req.event_id,
        req.round
    )
    .fetch_all(&auth_session.backend.db);

    let total_seats_fut = sqlx::query!(
        // language=PostgreSQL
        r#"
        SELECT max_users
        FROM round_max_users
        WHERE event_id = $1 AND round = $2;
        "#,
        req.event_id,
        req.round
    )
    .fetch_one(&auth_session.backend.db);

    // Await queries together for improved performance
    let (presences, total_seats) = match futures::try_join!(presences_fut, total_seats_fut) {
        Ok((presences, total_seats)) => (presences, total_seats.max_users),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Json(AdminPresenceResponse {
        name: event.name,
        room: event.room,
        total_seats,
        presences,
    })
    .into_response()
}
