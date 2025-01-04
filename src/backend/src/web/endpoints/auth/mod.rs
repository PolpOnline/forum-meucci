mod callback;
mod login;
mod logout;

use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes![callback::google_oauth_callback_handler])
        .routes(routes![login::google_login_handler])
        .routes(routes![logout::logout_handler])
}
