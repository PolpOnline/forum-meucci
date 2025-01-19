use axum::{extract::Request, middleware::Next, response::IntoResponse};
use http::StatusCode;

use crate::users::AuthSession;

pub async fn end_bookings_routing(
    auth_session: AuthSession,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let now = chrono::Utc::now();

    if now > auth_session.backend.config.bookings_end_date {
        return StatusCode::GONE.into_response();
    }

    next.run(request).await
}
