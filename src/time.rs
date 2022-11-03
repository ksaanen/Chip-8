use std::time::{SystemTime, Duration};

struct Timer {
    TIME_STAMP: SystemTime
}

impl Timer {
    pub fn new() -> Timer {
        return Timer { TIME_STAMP: SystemTime::now()}
    }

    pub fn time_passed(self) -> Duration {
        let now = SystemTime::now();
        let diff = now.duration_since(self.TIME_STAMP).unwrap();
        return diff;
    }
}

