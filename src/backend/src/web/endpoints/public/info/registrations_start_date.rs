use axum::{response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{app::openapi::INFO_TAG, users::AuthSession};

#[derive(Serialize, ToSchema)]
pub struct RegistrationsStartDateResponse {
    registrations_start_date: DateTime<Utc>,
}

#[utoipa::path(
    get,
    path = "/registrations_start_date",
    summary = "Registrations Start Date",
    responses(
        (status = OK, description = "Returns the start date of the registrations", body = RegistrationsStartDateResponse),
    ),
    tag = INFO_TAG,
)]
pub async fn registrations_start_date(auth_session: AuthSession) -> impl IntoResponse {
    let registrations_start_date = auth_session.backend.config.registrations_start_date;

    Json(RegistrationsStartDateResponse {
        registrations_start_date,
    })
}
