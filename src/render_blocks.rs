use blue_engine::{ObjectSettings, ObjectStorage, Renderer, Vertex};
use crate::tetris_block::TetrisBlock;

pub fn render_blocks(blocks: Vec<&TetrisBlock>, objects: &mut ObjectStorage, renderer: &mut Renderer) {
    objects.clear();
    blocks.iter().for_each(|block| {
        objects.new_object(
            String::from(&block.id),
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
                    position: [0.2, 0.1, 0.0],
                    uv: [0.0, 1.0],
                    normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                    position: [0.2, 0.0, 0.0],
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

        objects.get_mut(&block.id).unwrap().set_position((-1.0)+(block.x as f32/5.0), (-1.0)+(block.y as f32/10.0), 0.0);
    });

}