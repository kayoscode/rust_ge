use std::{time::SystemTime};

/// Class holding information about a start and stoptime.
/// The timer trait can be used to get duration information.
pub struct Stopwatch {
    start_time: SystemTime,
    stop_time: SystemTime,
}

pub trait Timer {
    /// Starts the timer.
    fn start(&mut self);

    /// Stops the timer.
    fn stop(&mut self);

    /// The total number of elapsed time in milliseconds.
    fn elapsed_millis(&self) -> i64;

    /// The total number of elapsed time in nanoseconds.
    fn elapsed_nanos(&self) -> i64;

    /// The total number of elapsed time in microseconds.
    fn elapsed_micros(&self) -> i64;

    /// The total number of elapsed time in seconds as a float.
    fn elapsed_seconds(&self) -> f64;
}

impl Stopwatch {
    pub fn new() -> Stopwatch {
        Stopwatch {
            start_time: SystemTime::now(),
            stop_time: SystemTime::now()
        }
    }
}

impl Timer for Stopwatch {
    fn start(&mut self) {
        self.start_time = SystemTime::now();
        self.stop_time = self.start_time.clone();
    }

    fn stop(&mut self) {
        self.stop_time = SystemTime::now();
    }

    fn elapsed_millis(&self) -> i64 {
        if let Ok(duration) = self.stop_time.duration_since(self.start_time) {
            return duration.as_millis() as i64;
        }

        0
    }

    fn elapsed_nanos(&self) -> i64 {
        if let Ok(duration) = self.stop_time.duration_since(self.start_time) {
            return duration.as_nanos() as i64;
        }

        0
    }

    fn elapsed_micros(&self) -> i64 {
        if let Ok(duration) = self.stop_time.duration_since(self.start_time) {
            return duration.as_micros() as i64;
        }

        0
    }

    fn elapsed_seconds(&self) -> f64 {
        if let Ok(duration) = self.stop_time.duration_since(self.start_time) {
            return duration.as_secs_f64()
        }

        0.0
    }
}