mod game_engine_setup;
mod tetris_block;
mod render_block;
mod tetris_engine;

use std::time::Duration;
use crate::game_engine_setup::{FramerateLimiter, setup_engine};
use crate::render_block::render_block;
use crate::tetris_block::TetrisBlock;
use crate::tetris_engine::TetrisEngine;

fn main() {
    let rendering_engine = setup_engine();

    let tetris_engine = TetrisEngine::new();

    let mut game_speed = FramerateLimiter::new(Duration::from_secs(1));
    rendering_engine
        .update_loop(move |renderer, _window, objects, _input, _camera, _plugins| {
            game_speed.tick(||{
                objects.clear();

                tetris_engine.generate_blocks().iter().for_each(|block|{
                    render_block(block, objects, renderer);
                })
            })
        })
        .expect("Error during update loop");
}



