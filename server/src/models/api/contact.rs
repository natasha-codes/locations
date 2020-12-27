use serde::Serialize;

use super::ExternallyExposed;

#[derive(Serialize)]
pub struct Contact {}

impl Contact {
    pub fn new() -> Self {
        Contact {}
    }
}

impl ExternallyExposed for Contact {}
