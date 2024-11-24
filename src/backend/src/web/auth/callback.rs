use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
};
use http::StatusCode;
use oauth2::CsrfToken;
use serde::Deserialize;
use tower_sessions::Session;
use utoipa::IntoParams;

use crate::{app::AUTH_TAG, models::credentials::Credentials, users::AuthSession};

#[derive(Deserialize, IntoParams)]
pub(super) struct AuthzResp {
    code: String,
    #[param(value_type = String)]
    state: CsrfToken,
}

//noinspection RsLiveness
#[utoipa::path(
    get,
    path = "/callback",
    params(AuthzResp),
    responses(
        (status = 303, description = "Redirect to Auth Success page", headers(
            ("Set-Cookie" = String, description = "Session cookie")
        )),
        (status = BAD_REQUEST, description = "csrf_state not found in session"),
        (status = UNAUTHORIZED, description = "Invalid CSRF state"),
        (status = INTERNAL_SERVER_ERROR, description = "Failed to authenticate user")
    ),
    tag = AUTH_TAG
)]
pub(super) async fn google_oauth_callback_handler(
    mut auth_session: AuthSession,
    session: Session,
    Query(AuthzResp {
        code,
        state: new_state,
    }): Query<AuthzResp>,
) -> impl IntoResponse {
    let Ok(Some(old_state)) = session.get("csrf_state").await else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let creds = Credentials {
        code,
        old_state,
        new_state,
    };

    let user = match auth_session.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (StatusCode::UNAUTHORIZED, "Invalid CSRF state".to_string()).into_response()
        }
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("/auth/login-success").into_response()
}
