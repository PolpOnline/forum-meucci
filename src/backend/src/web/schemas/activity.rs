use chrono::{DateTime, Utc};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::app::config::Config;

#[derive(Serialize, ToSchema, restructed::Models)]
#[view(ActivityWithoutDate, omit(date), derive(Default))]
pub struct Activity {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = 0, minimum = 0)]
    /// The round of the activity (0-indexed)
    pub round: i32,
    #[schema(example = "Activity 1")]
    pub name: String,
    #[schema(example = "This is the description of activity 1")]
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

impl Activity {
    pub fn from_without_date(
        activity: ActivityWithoutDate,
        config: &Config,
    ) -> Result<Self, RoundConversionError> {
        Ok(Activity {
            id: activity.id,
            round: activity.round,
            name: activity.name,
            description: activity.description,
            room: activity.room,
            used_seats: activity.used_seats,
            total_seats: activity.total_seats,
            present: activity.present,
            date: round_to_date(config, activity.round)?,
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
