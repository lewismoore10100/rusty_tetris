use std::thread;
use std::time::{Duration, Instant};

use blue_engine::{Engine, WindowDescriptor};

pub struct SpeedLimiter {
    duration_between_action: Duration,
    last_time: Instant,
}

impl SpeedLimiter {
    pub fn new(frames_per_duration: Duration) -> SpeedLimiter {
        SpeedLimiter {
            duration_between_action: frames_per_duration,
            last_time: Instant::now(),
        }
    }
    pub fn tick<F: FnMut() -> ()>(&mut self, mut f: F) {
        if Instant::now() - self.last_time >= self.duration_between_action {
            f();
            self.last_time = Instant::now();
        }
    }
}

pub fn setup_engine() -> Engine {
    return Engine::new_config(WindowDescriptor {
        width: 400,
        height: 800,
        title: "Rusty Tetris",
        ..Default::default()
    }).expect("engine couldn't be initialized");
}