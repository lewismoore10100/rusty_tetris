use std::thread;
use std::time::{Duration, Instant};

use blue_engine::{Engine, WindowDescriptor};

pub struct FramerateLimiter {
    frames_per_duration: Duration
}

impl FramerateLimiter {
    pub fn new(frames_per_duration: Duration) -> FramerateLimiter {
        FramerateLimiter {
            frames_per_duration
        }
    }
    pub fn tick<F: FnMut() -> ()>(&self, mut f: F){
        thread::sleep(self.frames_per_duration);
        f();
    }
}

pub struct GameProgressLimiter {
    grame_progress_duration: Duration,
    last_time: Instant
}

impl GameProgressLimiter {
    pub fn new(frames_per_duration: Duration) -> GameProgressLimiter {
        GameProgressLimiter {
            grame_progress_duration: frames_per_duration,
            last_time: Instant::now(),
        }
    }
    pub fn tick<F: FnMut() -> ()>(&mut self, mut f: F){
        let current_time = Instant::now();
        if current_time - self.last_time >= self.grame_progress_duration {
            self.last_time = current_time;
            f();
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