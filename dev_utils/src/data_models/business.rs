use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Business {
    anzsic16: String,
    area: String,
    year: u32,
    geo_count: u32,
    ec_count: u32,
}
