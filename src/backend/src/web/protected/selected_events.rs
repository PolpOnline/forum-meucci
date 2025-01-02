use ahash::AHashMap;
use axum::{response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    app::{config::Config, openapi::EVENT_TAG},
    users::AuthSession,
};

#[derive(Serialize, ToSchema)]
pub struct SelectedEventResponse {
    events: Vec<SelectedEvent>,
}

#[derive(Default)]
struct SelectedEventWithoutDate {
    id: i32,
    round: i32,
    name: String,
    description: Option<String>,
    room: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct SelectedEvent {
    #[schema(example = 1)]
    id: i32,
    #[schema(example = 0, minimum = 0)]
    /// The round of the event (0-indexed)
    round: i32,
    #[schema(example = "Event 1")]
    name: String,
    #[schema(example = "This is the description of event 1")]
    description: Option<String>,
    #[schema(example = "Room 1")]
    room: Option<String>,
    date: DateTime<Utc>,
}

#[utoipa::path(
    get,
    path = "/selected_events",
    responses(
        (status = OK, description = "Returns the selected events", body = SelectedEventResponse),
        (status = UNAUTHORIZED, description = "Not logged in"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("session" = [])
    ),
    tag = EVENT_TAG,
)]
pub(super) async fn selected_events(auth_session: AuthSession) -> impl IntoResponse {
    let user_id = match auth_session.user {
        Some(user) => user.id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let events = match sqlx::query_as!(
        SelectedEventWithoutDate,
        // language=PostgreSQL
        r#"
         SELECT event.id            AS id,
                event_user.round    AS round,
                event.name          AS name,
                event.description   AS description,
                event.room          AS room
         FROM event_user
                  JOIN event ON event_user.event_id = event.id
         WHERE event_user.user_id = $1
         ORDER BY event_user.round;
        "#,
        user_id
    )
    .fetch_all(&auth_session.backend.db)
    .await
    {
        Ok(r) => r,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let num_rounds = auth_session.backend.config.get_num_rounds();

    let events = fill_gaps(events, num_rounds);

    let events = events
        .into_iter()
        .map(|event| SelectedEvent::from_without_date(event, &auth_session.backend.config))
        .collect();

    Json(SelectedEventResponse { events }).into_response()
}

fn fill_gaps(
    events: Vec<SelectedEventWithoutDate>,
    num_rounds: usize,
) -> Vec<SelectedEventWithoutDate> {
    let round_map: AHashMap<i32, &SelectedEventWithoutDate> =
        events.iter().map(|event| (event.round, event)).collect();

    let mut filled_events = Vec::with_capacity(num_rounds);
    // Iterate from 0 up to num_rounds - 1
    for round in 0..num_rounds as i32 {
        if let Some(existing) = round_map.get(&round) {
            filled_events.push(SelectedEventWithoutDate {
                id: existing.id,
                round: existing.round,
                name: existing.name.clone(),
                description: existing.description.clone(),
                room: existing.room.clone(),
            });
        } else {
            filled_events.push(SelectedEventWithoutDate {
                round,
                ..Default::default()
            });
        }
    }

    filled_events
}

impl SelectedEvent {
    fn from_without_date(event: SelectedEventWithoutDate, config: &Config) -> Self {
        let date_map = &config.date_map;

        SelectedEvent {
            id: event.id,
            round: event.round,
            name: event.name,
            description: event.description,
            room: event.room,
            // intentionally crash if the round is not in the map
            date: *date_map.get(event.round as usize).unwrap(),
        }
    }
}
