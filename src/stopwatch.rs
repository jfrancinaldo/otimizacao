//! Useful for benchmarking

#![allow(unused)]

use std::time::{Duration, Instant};

pub struct Stopwatch {
    start_time: Instant,
}

impl Stopwatch {
    pub fn start() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }
}
