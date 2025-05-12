use openidconnect::{
    core::{
        CoreAuthDisplay, CoreClaimName, CoreClaimType, CoreClientAuthMethod, CoreGrantType,
        CoreJsonWebKey, CoreJweContentEncryptionAlgorithm, CoreJweKeyManagementAlgorithm,
        CoreResponseMode, CoreResponseType, CoreSubjectIdentifierType,
    }, AdditionalProviderMetadata, ClientId, ClientSecret, EndpointMaybeSet,
    EndpointNotSet, EndpointSet, IssuerUrl, ProviderMetadata, RedirectUrl,
    RevocationUrl,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RevocationEndpointProviderMetadata {
    revocation_endpoint: String,
}

impl AdditionalProviderMetadata for RevocationEndpointProviderMetadata {}

type GoogleProviderMetadata = ProviderMetadata<
    RevocationEndpointProviderMetadata,
    CoreAuthDisplay,
    CoreClientAuthMethod,
    CoreClaimName,
    CoreClaimType,
    CoreGrantType,
    CoreJweContentEncryptionAlgorithm,
    CoreJweKeyManagementAlgorithm,
    CoreJsonWebKey,
    CoreResponseMode,
    CoreResponseType,
    CoreSubjectIdentifierType,
>;

pub type CoreClient = openidconnect::core::CoreClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

pub async fn build_google_oauth_client(
    redirect_uri: String,
    unredirectable_async_http_client: &reqwest::Client,
) -> color_eyre::Result<CoreClient> {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").expect("Missing GOOGLE_CLIENT_ID!");
    let client_id = ClientId::new(client_id.to_string());
    let client_secret =
        std::env::var("GOOGLE_CLIENT_SECRET").expect("Missing GOOGLE_CLIENT_SECRET!");
    let client_secret = ClientSecret::new(client_secret.to_string());
    let issuer_url =
        IssuerUrl::new("https://accounts.google.com".to_string()).expect("Invalid issuer URL");

    let provider_metadata =
        GoogleProviderMetadata::discover_async(issuer_url, unredirectable_async_http_client)
            .await
            .expect("Failed to discover Google provider metadata");

    let revocation_endpoint = provider_metadata
        .additional_metadata()
        .revocation_endpoint
        .clone();

    let client = openidconnect::core::CoreClient::from_provider_metadata(
        provider_metadata,
        client_id,
        Some(client_secret),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri).expect("Invalid redirect URL"))
    .set_revocation_url(
        RevocationUrl::new(revocation_endpoint).expect("Invalid revocation endpoint URL"),
    );

    Ok(client)
}
