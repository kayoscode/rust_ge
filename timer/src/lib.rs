use std::{time::SystemTime};

/// Class holding information about a start and stoptime.
/// The timer trait can be used to get duration information.
pub struct Stopwatch {
    start_time: SystemTime,
    stop_time: SystemTime,
}

impl Stopwatch {
    pub fn new() -> Stopwatch {
        Stopwatch {
            start_time: SystemTime::now(),
            stop_time: SystemTime::now()
        }
    }

    pub fn start(&mut self) {
        self.start_time = SystemTime::now();
        self.stop_time = self.start_time.clone();
    }

    pub fn stop(&mut self) {
        self.stop_time = SystemTime::now();
    }

    pub fn elapsed_millis(&self) -> i64 {
        if let Ok(duration) = self.stop_time.duration_since(self.start_time) {
            return duration.as_millis() as i64;
        }

        0
    }

    pub fn elapsed_nanos(&self) -> i64 {
        if let Ok(duration) = self.stop_time.duration_since(self.start_time) {
            return duration.as_nanos() as i64;
        }

        0
    }

    pub fn elapsed_micros(&self) -> i64 {
        if let Ok(duration) = self.stop_time.duration_since(self.start_time) {
            return duration.as_micros() as i64;
        }

        0
    }

    pub fn elapsed_seconds(&self) -> f64 {
        if let Ok(duration) = self.stop_time.duration_since(self.start_time) {
            return duration.as_secs_f64()
        }

        0.0
    }

}