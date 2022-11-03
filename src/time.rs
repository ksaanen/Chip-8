use std::time::{SystemTime, Duration};

pub struct Time {
    TIME_STAMP: SystemTime
}

impl Time {
    pub fn new() -> Time {
        return Time { TIME_STAMP: SystemTime::now()}
    }

    pub fn time_passed(self) -> Duration {
        let now = SystemTime::now();
        let diff = now.duration_since(self.TIME_STAMP).unwrap();
        return diff;
    }
}

