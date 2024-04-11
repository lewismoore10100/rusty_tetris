use std::time::Duration;
use blue_engine::KeyCode;

use crate::game_engine_setup::{FramerateLimiter, GameProgressLimiter, setup_engine};
use crate::render_blocks::render_blocks;
use crate::tetris_engine::TetrisEngine;

mod game_engine_setup;
mod tetris_block;
mod render_blocks;
mod tetris_engine;
mod square;

fn main() {
    let rendering_engine = setup_engine();

    let mut tetris_engine = TetrisEngine::new();

    let mut frame_rate_limiter = FramerateLimiter::new(Duration::from_secs(1/60));
    let mut game_progress_limiter = GameProgressLimiter::new(Duration::from_secs(1));
    rendering_engine
        .update_loop(move |renderer, _window, objects, input, _camera, _plugins| {
            if input.key_pressed(KeyCode::ArrowLeft) {
                tetris_engine.move_left();
            }

            if input.key_pressed(KeyCode::ArrowRight) {
                tetris_engine.move_right();
            }

            if input.key_pressed(KeyCode::ArrowDown) {
                tetris_engine.drop();
            }

            render_blocks(tetris_engine.generate_blocks(), objects, renderer);

            frame_rate_limiter.tick(||{
                game_progress_limiter.tick(||{
                    tetris_engine.tick();
                })
            })
        })
        .expect("Error during update loop");
}



