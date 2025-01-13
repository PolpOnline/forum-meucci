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
    pub room: String,
    pub used_seats: i64,
    pub total_seats: i64,
    pub present: bool,
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
    pub room: String,
    #[schema(example = 10, minimum = 0)]
    pub used_seats: i64,
    #[schema(example = 20, minimum = 0)]
    pub total_seats: i64,
    #[schema(example = true)]
    pub present: bool,
    pub date: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum RoundConversionError {
    #[error("Round `{0}` is not mapped to date")]
    RoundNotInDateMap(i32),
}

impl Event {
    pub fn from_without_date(
        event: EventWithoutDate,
        config: &Config,
    ) -> Result<Self, RoundConversionError> {
        Ok(Event {
            id: event.id,
            round: event.round,
            name: event.name,
            description: event.description,
            room: event.room,
            used_seats: event.used_seats,
            total_seats: event.total_seats,
            present: event.present,
            date: round_to_date(config, event.round)?,
        })
    }
}

pub fn round_to_date(config: &Config, round: i32) -> Result<DateTime<Utc>, RoundConversionError> {
    let date_map = &config.date_map;

    match date_map.get(round as usize) {
        Some(date) => Ok(*date),
        None => Err(RoundConversionError::RoundNotInDateMap(round)),
    }
}
