use std::time::Duration;

use crate::game_engine_setup::{FramerateLimiter, GameProgressLimiter, setup_engine};
use crate::render_block::render_block;
use crate::tetris_engine::TetrisEngine;

mod game_engine_setup;
mod tetris_block;
mod render_block;
mod tetris_engine;
mod square;

fn main() {
    let rendering_engine = setup_engine();

    let mut tetris_engine = TetrisEngine::new();

    let mut frame_rate_limiter = FramerateLimiter::new(Duration::from_secs(1/60));
    let mut game_progress_limiter = GameProgressLimiter::new(Duration::from_secs(1));
    rendering_engine
        .update_loop(move |renderer, _window, objects, _input, _camera, _plugins| {
            frame_rate_limiter.tick(||{
                game_progress_limiter.tick(||{
                    tetris_engine.generate_blocks().iter().for_each(|block|{
                        render_block(block, objects, renderer);
                    });
                    tetris_engine.tick();
                })
            })
        })
        .expect("Error during update loop");
}



