use ahash::AHashMap;
use axum::{response::IntoResponse, Json};
use http::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    app::openapi::EVENT_TAG,
    users::AuthSession,
    web::protected::schemas::event::{Event, EventWithoutDate},
};

#[derive(Serialize, ToSchema)]
pub struct SelectedEventResponse {
    events: Vec<Event>,
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
        EventWithoutDate,
        // language=PostgreSQL
        r#"
         SELECT event.id            AS id,
                event_user.round    AS round,
                event.name          AS name,
                event.description   AS description,
                event.room          AS room,
                (round_max_users.max_users -  COUNT(event_user.user_id)) AS available_seats,
                round_max_users.max_users AS total_seats
         FROM event_user
                JOIN event ON event_user.event_id = event.id
                JOIN round_max_users ON event.id = round_max_users.event_id AND event_user.round = round_max_users.round
         WHERE event_user.user_id = $1
         GROUP BY event.id, event_user.round, event.name, event.description, event.room, round_max_users.max_users
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

    let events: Result<Vec<_>, _> = events
        .into_iter()
        .map(|event| Event::from_without_date(event, &auth_session.backend.config))
        .collect();

    let events = match events {
        Ok(events) => events,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    Json(SelectedEventResponse { events }).into_response()
}

fn fill_gaps(events: Vec<EventWithoutDate>, num_rounds: usize) -> Vec<EventWithoutDate> {
    let round_map: AHashMap<i32, &EventWithoutDate> =
        events.iter().map(|event| (event.round, event)).collect();

    let mut filled_events = Vec::with_capacity(num_rounds);
    // Iterate from 0 up to num_rounds - 1
    for round in 0..num_rounds as i32 {
        if let Some(existing) = round_map.get(&round) {
            filled_events.push(EventWithoutDate {
                id: existing.id,
                round: existing.round,
                name: existing.name.clone(),
                description: existing.description.clone(),
                room: existing.room.clone(),
                available_seats: existing.available_seats,
                total_seats: existing.total_seats,
            });
        } else {
            filled_events.push(EventWithoutDate {
                round,
                ..Default::default()
            });
        }
    }

    filled_events
}
