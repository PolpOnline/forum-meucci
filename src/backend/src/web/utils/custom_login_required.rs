#[macro_export]
macro_rules! custom_login_required {
    ($backend_type:ty, $alternative:expr) => {{
        async fn is_authenticated(auth_session: axum_login::AuthSession<$backend_type>) -> bool {
            auth_session.user.is_some()
        }

        axum_login::predicate_required!(is_authenticated, $alternative)
    }};
}
