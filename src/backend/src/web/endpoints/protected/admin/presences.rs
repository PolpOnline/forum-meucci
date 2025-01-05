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
    presences: Vec<Presence>,
}

#[derive(Serialize, ToSchema)]
pub struct Presence {
    #[schema(example = 0, minimum = 0)]
    name: String,
    #[schema(example = 0, minimum = 0)]
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

    let presences = match sqlx::query!(
        // language=PostgreSQL
        r#"
        SELECT COALESCE("user".name, "user".email) AS name,
               event_user.joined_at IS NOT NULL AS present
        FROM event_user
                 JOIN "user" ON event_user.user_id = "user".id
        WHERE event_user.event_id = $1 AND event_user.round = $2
        ORDER BY name;
        "#,
        req.event_id,
        req.round
    )
    .fetch_all(&auth_session.backend.db)
    .await
    {
        Ok(r) => r,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let presences: Vec<_> = presences
        .into_iter()
        .map(|r| Presence {
            name: r.name.unwrap(),
            present: r.present.unwrap(),
        })
        .collect();

    Json(AdminPresenceResponse {
        name: event.name,
        room: event.room,
        presences,
    })
    .into_response()
}
