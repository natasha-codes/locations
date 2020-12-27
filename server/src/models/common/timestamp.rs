use std::time::{SystemTime, UNIX_EPOCH};

pub type Timestamp = u64;

/// Returns a Timestamp representing "now" in epoch seconds.
pub fn timestamp_now() -> Timestamp {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime::now() is prior to the UNIX_EPOCH")
        .as_secs()
}
