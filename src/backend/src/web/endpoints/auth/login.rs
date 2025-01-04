use axum::response::{IntoResponse, Redirect};
use tower_sessions::Session;

use crate::{app::openapi::AUTH_TAG, users::AuthSession};

#[utoipa::path(
    get,
    path = "/login",
    responses((status = 303, description = "Redirect to Google OAuth")),
    tag = AUTH_TAG
)]
pub(in crate::web) async fn google_login_handler(
    auth_session: AuthSession,
    session: Session,
) -> impl IntoResponse {
    let (authorize_url, csrf_state) = auth_session.backend.authorize_url();

    // Store the csrf_state in the session, so we can assert equality in the
    // callback
    session
        .insert("csrf_state", csrf_state.secret())
        .await
        .expect("Failed to insert csrf_state into session");

    // Redirect to your oauth service
    Redirect::to(authorize_url.as_str())
}
