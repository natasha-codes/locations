use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Coordinate {
    pub hour: u8,
    pub min: u8,
    pub second: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub lat: Coordinate,
    pub long: Coordinate,
}
