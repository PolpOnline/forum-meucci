use crate::{models::user::ForumUserRole, users::AuthSession};

pub struct ActivityBasicInfo {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) room: String,
}

/// Get a single activity for an admin user
pub async fn get_admin_activity(
    auth_session: &AuthSession,
    activity_id: i32,
) -> Result<Option<ActivityBasicInfo>, sqlx::Error> {
    // Intentionally allow access to activities that should not be displayed
    let name = sqlx::query_as!(
        ActivityBasicInfo,
        r#"
        SELECT name, description, room
        FROM forum_activity
        WHERE forum_activity.id = $1
        "#,
        activity_id
    )
    .fetch_one(&auth_session.backend.db)
    .await?;

    Ok(Some(name))
}

/// Get a single activity for a user (only activities they are hosting, checking
/// the activity_admin table)
pub async fn get_host_activity(
    auth_session: &AuthSession,
    user_id: i32,
    activity_id: i32,
) -> Result<Option<ActivityBasicInfo>, sqlx::Error> {
    // Disallow access to activities that should not be displayed
    // Get the name of the activity while checking if the user has access to it
    let name = sqlx::query_as!(
        ActivityBasicInfo,
        r#"
        SELECT name, description, room
        FROM forum_activity
        JOIN forum_activity_host ON forum_activity.id = forum_activity_host.activity_id
        WHERE forum_activity_host.user_id = $1 AND forum_activity.id = $2 AND forum_activity.should_display IS TRUE
        "#,
        user_id,
        activity_id
    )
    .fetch_optional(&auth_session.backend.db)
    .await?;

    Ok(name)
}

/// Get a single activity for a user that is already verified as admin
/// DO NOT PASS NON-ADMIN/HOST USERS TO THIS FUNCTION
pub async fn get_activity(
    auth_session: &AuthSession,
    forum_role: ForumUserRole,
    user_id: i32,
    activity_id: i32,
) -> Result<Option<ActivityBasicInfo>, sqlx::Error> {
    match forum_role {
        ForumUserRole::Admin => get_admin_activity(auth_session, activity_id).await,
        ForumUserRole::Host => get_host_activity(auth_session, user_id, activity_id).await,
        ForumUserRole::Normal => unreachable!(),
    }
}

// Check if a user is an admin of an activity
pub async fn check_host_activity(
    auth_session: &AuthSession,
    user_id: i32,
    activity_id: i32,
) -> Result<bool, sqlx::Error> {
    let matches = sqlx::query!(
        r#"
        SELECT activity_id
        FROM forum_activity_host
        JOIN forum_activity ON forum_activity_host.activity_id = forum_activity.id
        WHERE user_id = $1 AND activity_id = $2 AND should_display IS TRUE
        "#,
        user_id,
        activity_id
    )
    .fetch_optional(&auth_session.backend.db)
    .await?;

    Ok(matches.is_some())
}

pub async fn user_has_access_to_activity(
    auth_session: &AuthSession,
    user_role: ForumUserRole,
    user_id: i32,
    activity_id: i32,
) -> Result<bool, sqlx::Error> {
    let has_access = match user_role {
        ForumUserRole::Admin => true,
        ForumUserRole::Host => check_host_activity(auth_session, user_id, activity_id).await?,
        ForumUserRole::Normal => unreachable!(),
    };

    Ok(has_access)
}
