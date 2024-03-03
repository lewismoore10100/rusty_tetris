use std::time::Duration;

use crate::game_engine_setup::{FramerateLimiter, setup_engine};
use crate::render_block::render_block;
use crate::tetris_block::TetrisBlock;

mod game_engine_setup;
mod tetris_block;
mod render_block;

fn main() {
    let rendering_engine = setup_engine();

    let mut game_speed = FramerateLimiter::new(Duration::from_secs(1));
    rendering_engine
        .update_loop(move |renderer, _window, objects, _input, _camera, _plugins| {
            game_speed.tick(||{
                objects.clear();

                to_blocks().iter().for_each(|block|{
                    render_block(block, objects, renderer);
                })
            })
        })
        .expect("Error during update loop");
}

pub fn to_blocks() -> Vec<TetrisBlock> {
    vec![TetrisBlock{x: 0, y:0}, TetrisBlock{x: 2, y: 0}]
}



