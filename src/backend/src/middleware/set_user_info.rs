use axum::{extract::Request, middleware::Next, response::Response};
use http::{header, HeaderValue};
use tower_sessions::cookie::CookieBuilder;

use crate::users::AuthSession;

const USER_EMAIL_COOKIE: &str = "meucci_forum_user_email";

pub async fn set_user_info(request: Request, next: Next) -> Response {
    let session = request.extensions().get::<AuthSession>();

    let user_email = if let Some(auth_session) = session {
        auth_session.user.as_ref().map(|user| user.email.clone())
    } else {
        None
    };

    let mut response = next.run(request).await;

    if let Some(user_email) = user_email {
        let cookie = CookieBuilder::new(USER_EMAIL_COOKIE, user_email)
            .http_only(true)
            .secure(true)
            .build();

        response.headers_mut().append(
            header::SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );
    }

    response
}
