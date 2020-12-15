use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Covid {
    direction: String,
    year: u16,
    date: String,
    weekday: String,
    current_match: String,
    country: String,
    commodity: String,
    transport_mode: String,
    measure: String,
    value: u128,
    cumulative: u128,
}
