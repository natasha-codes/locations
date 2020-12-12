use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Coordinate {
  pub hour: u8,
  pub min: u8,
  pub second: u8,
}

impl fmt::Display for Coordinate {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}° {}' {}\"", self.hour, self.min, self.second)
  }
}

#[derive(Serialize, Deserialize)]
pub struct Location {
  pub lat: Coordinate,
  pub long: Coordinate,
}

impl fmt::Display for Location {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} by {}", self.lat, self.long)
  }
}
