use std::collections::HashSet;

use blue_engine::{ObjectSettings, ObjectStorage, Renderer, Vertex};

use crate::tetris_block::TetrisBlock;

pub fn render_blocks(blocks: Vec<&TetrisBlock>, objects: &mut ObjectStorage, renderer: &mut Renderer) {
    let mut block_ids_to_render: HashSet<String> = HashSet::with_capacity(blocks.len());

    blocks.iter().for_each(|block| {
        block_ids_to_render.insert(String::from(&block.id));

        let block_in_object_store = match objects.get_mut(&block.id) {
            Some(b) => b,
            None => {
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
                objects.get_mut(&block.id).unwrap()
            }
        };
        block_in_object_store.set_position((-1.0) + (block.x as f32 / 5.0), (-1.0) + (block.y as f32 / 10.0), 0.0);
    });

    objects.retain(|b, o| block_ids_to_render.contains(b));
}