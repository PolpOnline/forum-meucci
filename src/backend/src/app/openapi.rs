use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};

pub const AUTH_TAG: &str = "Auth";
pub const SYSTEM_TAG: &str = "System";
pub const FORUM_ACTIVITIES_TAG: &str = "Forum Activities";
pub const FORUM_ADMIN_TAG: &str = "Forum Admin";
pub const FORUM_INFO_TAG: &str = "Forum Info";

#[derive(OpenApi)]
#[openapi(
    modifiers(&ApiDocSecurityAddon),
    tags(
        (name = AUTH_TAG, description = "Endpoints to authenticate users"),
        (name = SYSTEM_TAG, description = "Endpoints to monitor the system"),
        (name = FORUM_ACTIVITIES_TAG, description = "Endpoints related to activities"),
        (name = FORUM_ADMIN_TAG, description = "Endpoints for host/administrators of the activities"),
        (name = FORUM_INFO_TAG, description = "Endpoints to get public information about activities"),
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
