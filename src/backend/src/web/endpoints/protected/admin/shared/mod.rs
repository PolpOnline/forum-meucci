use crate::{models::user::UserType, users::AuthSession};

pub struct EventBasicInfo {
    pub(crate) name: String,
    pub(crate) room: String,
}

/// Get a single event for an admin user
pub async fn get_admin_event(
    auth_session: &AuthSession,
    event_id: i32,
) -> Result<Option<EventBasicInfo>, sqlx::Error> {
    // Intentionally allow access to events that should not be displayed
    let name = sqlx::query_as!(
        EventBasicInfo,
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

/// Get a single event for a user (only events they are hosting, checking the
/// event_admin table)
pub async fn get_host_event(
    auth_session: &AuthSession,
    user_id: i32,
    event_id: i32,
) -> Result<Option<EventBasicInfo>, sqlx::Error> {
    // Disallow access to events that should not be displayed
    // Get the name of the event while checking if the user has access to it
    let name = sqlx::query_as!(
        EventBasicInfo,
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

/// Get a single event for a user that is already verified as admin
/// DO NOT PASS NON-ADMIN/HOST USERS TO THIS FUNCTION
pub async fn get_event(
    auth_session: &AuthSession,
    user_type: UserType,
    user_id: i32,
    event_id: i32,
) -> Result<Option<EventBasicInfo>, sqlx::Error> {
    match user_type {
        UserType::Admin => get_admin_event(&auth_session, event_id).await,
        UserType::Host => get_host_event(&auth_session, user_id, event_id).await,
        UserType::Normal => unreachable!(),
    }
}

// Check if a user is an admin of an event
pub async fn check_host_event(
    auth_session: &AuthSession,
    user_id: i32,
    event_id: i32,
) -> Result<bool, sqlx::Error> {
    let matches = sqlx::query!(
        // language=PostgreSQL
        r#"
        SELECT event_id
        FROM event_admin
        JOIN event ON event_admin.event_id = event.id
        WHERE user_id = $1 AND event_id = $2 AND should_display IS TRUE
        "#,
        user_id,
        event_id
    )
    .fetch_optional(&auth_session.backend.db)
    .await?;

    Ok(matches.is_some())
}

pub async fn user_has_access_to_event(
    auth_session: &AuthSession,
    user_type: UserType,
    user_id: i32,
    event_id: i32,
) -> Result<bool, sqlx::Error> {
    let has_access = match user_type {
        UserType::Admin => true,
        UserType::Host => check_host_event(&auth_session, user_id, event_id).await?,
        UserType::Normal => unreachable!(),
    };

    Ok(has_access)
}
