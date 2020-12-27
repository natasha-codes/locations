use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Location {
    latitude: f64,
    longitude: f64,
}
