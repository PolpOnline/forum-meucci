use axum::{response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{app::openapi::INFO_TAG, users::AuthSession};

#[derive(Serialize, ToSchema)]
pub struct BookingsStartDateResponse {
    bookings_start_date: DateTime<Utc>,
}

#[utoipa::path(
    get,
    path = "/bookings_start_date",
    summary = "Bookings Start Date",
    responses(
        (status = OK, description = "Returns the start date of the bookings", body = BookingsStartDateResponse),
    ),
    tag = INFO_TAG,
)]
pub async fn bookings_start_date(auth_session: AuthSession) -> impl IntoResponse {
    let bookings_start_date = auth_session.backend.config.bookings_start_date;

    Json(BookingsStartDateResponse {
        bookings_start_date,
    })
}
