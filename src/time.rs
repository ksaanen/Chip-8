use std::time::{SystemTime, UNIX_EPOCH};

pub fn time_millis() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u32
}
