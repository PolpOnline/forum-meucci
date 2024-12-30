use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

pub const AUTH_TAG: &str = "Auth";
pub const SYSTEM_TAG: &str = "System";
pub const USER_TAG: &str = "User";
pub const EVENT_TAG: &str = "Event";

#[derive(OpenApi)]
#[openapi(
    modifiers(&ApiDocSecurityAddon),
    tags(
        (name = AUTH_TAG, description = "Endpoints to authenticate users"),
        (name = SYSTEM_TAG, description = "Endpoints to monitor the system"),
        (name = USER_TAG, description = "Endpoints related to users")
    )
)]
pub(super) struct ApiDoc;

struct ApiDocSecurityAddon;

impl Modify for ApiDocSecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "session",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("meucci_forum_id"))),
            )
        }
    }
}
