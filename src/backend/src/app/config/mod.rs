use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    /// A vec with the index as the round and the date of the round as the value
    pub(crate) date_map: Vec<DateTime<Utc>>,
    pub(crate) bookings_start_date: DateTime<Utc>,
}

impl Config {
    pub(crate) fn init() -> Config {
        let config = include_str!("../../../config/config.json");
        serde_json::from_str(config).unwrap()
    }

    pub(crate) fn get_num_rounds(&self) -> usize {
        self.date_map.len()
    }
}
