use axum::{extract::Path, response::IntoResponse};
use axum_serde::Sonic;
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    app::openapi::ADMIN_TAG,
    models::user::UserType,
    users::AuthSession,
    web::{endpoints::protected::admin::shared::get_activity, schemas::activity::round_to_date},
};

#[derive(Deserialize, IntoParams)]
pub struct AdminPresenceRequest {
    /// The ID of the activity
    #[param(example = 1)]
    activity_id: i32,
    /// The round number
    #[param(example = 1)]
    round: i32,
}

#[derive(Serialize, ToSchema)]
pub struct AdminPresenceResponse {
    #[schema(example = "Activity 1")]
    name: String,
    #[schema(example = "This is the description of activity 1")]
    description: String,
    #[schema(example = "Room 1")]
    room: String,
    #[schema(example = 20)]
    total_seats: i32,
    date: DateTime<Utc>,
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
    #[schema(example = false)]
    randomized: bool,
}

#[utoipa::path(
    get,
    path = "/presences/{activity_id}/{round}",
    summary = "Presences List",
    params(AdminPresenceRequest),
    responses(
        (status = OK, description = "List of the presences for a given activity and round", body = AdminPresenceResponse),
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

    let activity = get_activity(&auth_session, user_type, user_id, req.activity_id).await;

    let activity = match activity {
        Ok(Some(a)) => a,
        // User does not have access to the activity
        Ok(None) => return StatusCode::FORBIDDEN.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let presences_fut = sqlx::query_as!(
        Presence,
        r#"
        SELECT "user".id,
               "user".section,
               "user".class,
               COALESCE("user".name, "user".email) AS "name!: String",
               activity_user.joined_at IS NOT NULL AS "present!: bool",
               activity_user.randomized
        FROM activity_user
                 JOIN "user" ON activity_user.user_id = "user".id
        WHERE activity_user.activity_id = $1
          AND activity_user.round = $2
        ORDER BY name;
        "#,
        req.activity_id,
        req.round
    )
    .fetch_all(&auth_session.backend.db);

    let total_seats_fut = sqlx::query!(
        r#"
        SELECT max_users
        FROM round_max_users
        WHERE activity_id = $1 AND round = $2;
        "#,
        req.activity_id,
        req.round
    )
    .fetch_one(&auth_session.backend.db);

    // Await queries together for improved performance
    let (presences, total_seats) = match futures::try_join!(presences_fut, total_seats_fut) {
        Ok((presences, total_seats)) => (presences, total_seats.max_users),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let date = match round_to_date(&auth_session.backend.config, req.round) {
        Ok(date) => date,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Sonic(AdminPresenceResponse {
        name: activity.name,
        description: activity.description,
        room: activity.room,
        date,
        total_seats,
        presences,
    })
    .into_response()
}
