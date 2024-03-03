use std::time::Duration;

use blue_engine::{ObjectSettings, ObjectStorage, Renderer, Vertex};

use crate::game_engine_setup::{FramerateLimiter, setup_engine};
use crate::tetris_block::TetrisBlock;

mod game_engine_setup;
mod tetris_block;

fn main() {
    let rendering_engine = setup_engine();

    let mut game_speed = FramerateLimiter::new(Duration::from_secs(1));
    rendering_engine
        .update_loop(move |renderer, _window, objects, input, camera, plugins| {
            game_speed.tick(||{
                objects.clear();

                to_blocks().iter().for_each(|block|{
                    render_block(block, objects, renderer);
                })
            })
        })
        .expect("Error during update loop");
}




pub fn render_block(block: &TetrisBlock, objects: &mut ObjectStorage, renderer: &mut Renderer) {
    objects.new_object(
        block.name(),
        vec![
            Vertex {
                position: [0.0, 0.0, 0.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [0.0, 0.1, 0.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [0.1, 0.1, 0.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [0.1, 0.0, 0.0],
                uv: [0.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
        ],
        vec![2, 1, 0, 2, 0, 3],
        ObjectSettings {
            camera_effect: false,
            ..Default::default()
        },
        renderer,
    ).unwrap();
    let block_in_scene = objects.get_mut(&block.name()).unwrap();
    block_in_scene.set_position((-1.0)+(block.x as f32/10.0), (-1.0)+(block.y as f32/10.0), 0.0);

}
pub fn to_blocks() -> Vec<TetrisBlock> {
    vec![TetrisBlock{x: 0, y:0}, TetrisBlock{x: 2, y: 0}]
}



