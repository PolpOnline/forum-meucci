use async_trait::async_trait;
use axum_login::{AuthnBackend, UserId};
use openidconnect::{
    core::{CoreClient, CoreResponseType, CoreUserInfoClaims},
    reqwest::async_http_client,
    url::Url,
    AuthorizationCode, CsrfToken, Nonce, OAuth2TokenResponse, Scope,
};
use sqlx::PgPool;
use tokio::task;
use tracing::log::trace;

use crate::models::{credentials::Credentials, user::User};

#[derive(Debug, Clone)]
pub struct LoginBackend {
    db: PgPool,
    google_oauth_client: CoreClient,
}

impl LoginBackend {
    pub fn new(db: PgPool, client: CoreClient) -> Self {
        Self {
            db,
            google_oauth_client: client,
        }
    }

    pub fn authorize_url(&self) -> (Url, CsrfToken) {
        let (authorize_url, csrf_state, _nonce) = self
            .google_oauth_client
            .authorize_url(
                openidconnect::AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .add_extra_param("hd", std::env::var("EMAIL_DOMAIN").unwrap())
            .url();

        (authorize_url, csrf_state)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error(transparent)]
    Sqlx(sqlx::Error),

    #[error(transparent)]
    Reqwest(reqwest::Error),

    #[error(transparent)]
    OAuth2(color_eyre::eyre::Error),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

#[async_trait]
impl AuthnBackend for LoginBackend {
    type User = User;
    type Credentials = Credentials;
    type Error = BackendError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        if creds.old_state.secret() != creds.new_state.secret() {
            return Ok(None);
        };

        // Process authorization code, expecting a token response back
        let token = self
            .google_oauth_client
            .exchange_code(AuthorizationCode::new(creds.code))
            .request_async(async_http_client)
            .await
            .unwrap();

        let profile: CoreUserInfoClaims = self
            .google_oauth_client
            .user_info(token.access_token().clone(), None)
            .unwrap()
            .request_async(async_http_client)
            .await
            .unwrap();

        let email = profile.email().unwrap().to_string();
        let name = profile.name().unwrap().get(None).unwrap().to_string();
        trace!("Got email: {}", email);
        trace!("Got name: {}", name);

        self.google_oauth_client
            .revoke_token(token.access_token().into())
            .unwrap()
            .request_async(async_http_client)
            .await
            .expect("Failed to revoke token");

        let user: User = User::get_or_create_user_by_email(&self.db, email, name)
            .await
            .unwrap();

        Ok(Some(user))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        User::get_user_by_id(&self.db, user_id)
            .await
            .map_err(BackendError::Sqlx)
    }
}

pub type AuthSession = axum_login::AuthSession<LoginBackend>;
