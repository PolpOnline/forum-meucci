use chrono::{DateTime, Utc};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::app::config::Config;

#[derive(Default)]
pub struct EventWithoutDate {
    pub id: i32,
    pub round: i32,
    pub name: String,
    pub description: Option<String>,
    pub room: Option<String>,
    pub available_seats: Option<i64>,
    pub total_seats: i64,
}

#[derive(Serialize, ToSchema)]
pub struct Event {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = 0, minimum = 0)]
    /// The round of the event (0-indexed)
    pub round: i32,
    #[schema(example = "Event 1")]
    pub name: String,
    #[schema(example = "This is the description of event 1")]
    pub description: Option<String>,
    #[schema(example = "Room 1")]
    pub room: Option<String>,
    #[schema(example = 10, minimum = 0)]
    pub available_seats: Option<i64>,
    #[schema(example = 20, minimum = 0)]
    pub total_seats: i64,
    pub date: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum EventConversionError {
    #[error("Round `{0}` is not mapped to date")]
    RoundNotInDateMap(i32),
}

impl Event {
    pub fn from_without_date(
        event: EventWithoutDate,
        config: &Config,
    ) -> Result<Self, EventConversionError> {
        Ok(Event {
            id: event.id,
            round: event.round,
            name: event.name,
            description: event.description,
            room: event.room,
            available_seats: event.available_seats,
            total_seats: event.total_seats,
            date: round_to_date(config, event.round)?,
        })
    }
}

fn round_to_date(config: &Config, round: i32) -> Result<DateTime<Utc>, EventConversionError> {
    let date_map = &config.date_map;

    match date_map.get(round as usize) {
        Some(date) => Ok(*date),
        None => Err(EventConversionError::RoundNotInDateMap(round)),
    }
}
