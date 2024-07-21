use std::time::Instant;

pub const SECOND: u128 = 1000000000;

pub struct Time {
    start_time: Instant,
    delta: f64,
}

impl Time {
    pub fn new() -> Time {
        Time {
            start_time: Instant::now(),
            delta: 0.0,
        }
    }

    pub fn get_time(&self) -> u128 {
        self.start_time.elapsed().as_nanos()
    }

    pub fn get_delta(&self) -> f64 {
        self.delta
    }

    pub fn set_delta(&mut self, delta: f64) {
        self.delta = delta;
    }
}
