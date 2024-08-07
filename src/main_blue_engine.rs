use std::time::Duration;

use blue_engine::KeyCode;

use crate::game_engine_setup::{setup_engine, SpeedLimiter};
use crate::render_blocks::render_blocks;
use crate::tetris_engine::TetrisEngine;

mod game_engine_setup;
mod tetris_block;
mod render_blocks;
mod tetris_engine;
mod shapes;
mod rotation_position;
mod direction;
mod scoring;

fn main() {
    let title = "Rusty Tetris - Score: ";

    let rendering_engine = setup_engine();

    let mut tetris_engine = TetrisEngine::new();

    let mut frame_rate_limiter = SpeedLimiter::new(Duration::from_secs(1 / 60));
    let mut game_progress_limiter = SpeedLimiter::new(Duration::from_secs(1));
    rendering_engine
        .update_loop(move |renderer, window, objects, input, _camera, _plugins| {
            if input.key_pressed(KeyCode::ArrowLeft) {
                tetris_engine.move_left();
                render_blocks(tetris_engine.blocks_for_rendering(), objects, renderer);
            }

            if input.key_pressed(KeyCode::ArrowRight) {
                tetris_engine.move_right();
                render_blocks(tetris_engine.blocks_for_rendering(), objects, renderer);
            }

            if input.key_held(KeyCode::ArrowDown) {
                tetris_engine.move_down();
                render_blocks(tetris_engine.blocks_for_rendering(), objects, renderer);
            }

            if input.key_pressed(KeyCode::Space) {
                tetris_engine.drop();
                render_blocks(tetris_engine.blocks_for_rendering(), objects, renderer);
            }

            if input.key_pressed(KeyCode::ControlLeft) {
                tetris_engine.rotate();
                render_blocks(tetris_engine.blocks_for_rendering(), objects, renderer);
            }

            frame_rate_limiter.tick(|| {
                game_progress_limiter.tick(|| {
                    tetris_engine.tick();
                    render_blocks(tetris_engine.blocks_for_rendering(), objects, renderer);
                    let score_as_string = tetris_engine.score().to_string();
                    window.set_title(format!("{title}{score_as_string}").as_str());
                })
            })
        })
        .expect("Error during update loop");
}



