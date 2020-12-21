use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Location {
    latitude: f64,
    longitude: f64,
}
