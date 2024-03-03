use std::time::Duration;

use blue_engine::{ObjectSettings, ObjectStorage, Renderer, Vertex};

use crate::game_engine_setup::{FramerateLimiter, setup_engine};
use crate::tetris_block::TetrisBlock;

mod game_engine_setup;
mod tetris_block;

fn main() {
    let mut rendering_engine = setup_engine();

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




pub fn render_block(block: &TetrisBlock, mut objects: &mut ObjectStorage, renderer: &mut Renderer) {
    objects.new_object(
        block.name(),
        vec![
            Vertex {
                position: [0.2, 0.2, 0.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [0.2, -0.2, 0.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [-0.2, -0.2, 0.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [-0.2, 0.2, 0.0],
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
    block_in_scene.position.x += 1.0;
    block_in_scene.position.y += 1.0;

}
pub fn to_blocks() -> Vec<TetrisBlock> {
    vec![TetrisBlock{x: 0, y:0}]
}



