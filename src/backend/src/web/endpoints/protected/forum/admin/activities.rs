use axum::response::IntoResponse;
use axum_serde::Sonic;
use http::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{app::openapi::FORUM_ADMIN_TAG, models::user::ForumUserRole, users::AuthSession};

#[derive(Serialize, ToSchema)]
pub struct AdminActivityResponse {
    activities: Vec<AdminActivity>,
}

#[derive(Serialize, ToSchema)]
pub struct AdminActivity {
    #[schema(example = 1)]
    id: i32,
    #[schema(example = "Activity 1")]
    name: String,
    #[schema(example = "This is the description of activity 1")]
    description: String,
    #[schema(example = "Room 1")]
    room: String,
}

#[utoipa::path(
    get,
    path = "/activities",
    summary = "Activities List",
    responses(
        (status = OK, description = "List of the activities the user has access to, if admin all activities, if host only those they are hosting", body = AdminActivityResponse),
        (status = UNAUTHORIZED, description = "Not logged in"),
        (status = FORBIDDEN, description = "Not an admin or host"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error"),
        (status = 425, description = "Registrations have not started yet"),
    ),
    security(
        ("session" = [])
    ),
    tag = FORUM_ADMIN_TAG,
)]
pub(super) async fn activities(auth_session: AuthSession) -> impl IntoResponse {
    let (forum_role, user_id) = match auth_session.user {
        Some(ref user) => (user.forum_role, user.id),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let activities = match forum_role {
        ForumUserRole::Admin => admin_activities(&auth_session).await,
        ForumUserRole::Host => host_activities(&auth_session, user_id).await,
        ForumUserRole::Normal => return StatusCode::FORBIDDEN.into_response(),
    };

    let activities = match activities {
        Ok(r) => r,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Sonic(AdminActivityResponse { activities }).into_response()
}

/// Get all activities for an admin user (all activities)
async fn admin_activities(auth_session: &AuthSession) -> Result<Vec<AdminActivity>, sqlx::Error> {
    let activities = sqlx::query_as!(
        AdminActivity,
        r#"
        SELECT id,
               name,
               description,
               room
        FROM forum_activity
        WHERE should_display IS TRUE
        ORDER BY name;
        "#
    )
    .fetch_all(&auth_session.backend.db)
    .await?;

    Ok(activities)
}

/// Get all activities for a host user (only activities they are hosting,
/// checking the activity_admin table)
async fn host_activities(
    auth_session: &AuthSession,
    user_id: i32,
) -> Result<Vec<AdminActivity>, sqlx::Error> {
    let activities = sqlx::query_as!(
        AdminActivity,
        r#"
        SELECT id,
               name,
               description,
               room
        FROM forum_activity
                 JOIN forum_activity_host ON forum_activity.id = forum_activity_host.activity_id
        WHERE should_display IS TRUE AND user_id = $1
        ORDER BY name;
        "#,
        user_id
    )
    .fetch_all(&auth_session.backend.db)
    .await?;

    Ok(activities)
}
