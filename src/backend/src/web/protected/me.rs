use axum::response::IntoResponse;
use http::StatusCode;

use crate::users::AuthSession;

#[utoipa::path(
    get,
    path = "/me",
    responses(
        (status = 200, description = "Returns the user's name"),
        (status = 401, description = "Not logged in")
    )
)]
pub(super) async fn me(auth_session: AuthSession) -> impl IntoResponse {
    if let Some(user) = auth_session.user {
        let res = format!("You are {:?} <{:?}>", user.name, user.email);

        return (StatusCode::OK, res).into_response();
    }

    (StatusCode::UNAUTHORIZED, "Not logged in").into_response()
}
