use std::str::FromStr;

use serde::{Deserialize, Deserializer};

pub fn space_deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let str_sequence = String::deserialize(deserializer)?;

    if str_sequence.is_empty() {
        return Ok(Vec::with_capacity(0));
    }

    let result = str_sequence
        .trim()
        .split(' ')
        .map(|s| s.parse::<T>())
        .collect::<Result<Vec<T>, _>>()
        .map_err(serde::de::Error::custom)?;

    Ok(result)
}
