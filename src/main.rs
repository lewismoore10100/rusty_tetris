use std::time::{Duration, Instant};

use blue_engine::{header::Engine, WindowDescriptor};

fn main() {
    let mut rendering_engine = setup_engine();

    let mut game_speed = FramerateLimiter::new(Duration::from_secs(1));
    rendering_engine
        .update_loop(move |renderer, _window, objects, input, camera, plugins| {
            game_speed.tick(||{
                println!("tick");
            })
        })
        .expect("Error during update loop");
}

struct FramerateLimiter {
    last_time : Instant,
    tick_count: i64,
    frames_per_duration: Duration
}

impl FramerateLimiter {
    pub fn new(frames_per_duration: Duration) -> FramerateLimiter {
        FramerateLimiter {
            last_time: Instant::now(),
            tick_count: 0,
            frames_per_duration
        }
    }
    pub fn tick(&mut self, f: fn()){
        let current_time = Instant::now();
        if current_time - self.last_time >= self.frames_per_duration {
            self.tick_count += 1;
            self.last_time = current_time;
            f();
        }
    }
}

fn setup_engine() -> Engine {
     return Engine::new_config(WindowDescriptor {
        width: 600,
        height: 800,
        title: "Rusty Tetris",
        ..Default::default()
    }).expect("engine couldn't be initialized");
}