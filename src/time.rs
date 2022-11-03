use std::time::{SystemTime, Duration};

pub struct Time {
    time_stamp: SystemTime,
    pub running: bool
}

impl Time {
    pub fn new() -> Time {
        return Time {
            time_stamp: SystemTime::now(),
            running: true
        }
    }

    pub fn time_passed(&self) -> Duration {
        let now = SystemTime::now();
        let diff = now.duration_since(self.time_stamp).unwrap();
        return diff;
    }

    pub fn reset(&mut self) {
        self.time_stamp = SystemTime::now();
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}

