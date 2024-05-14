use std::collections::HashSet;

use blue_engine::{ObjectSettings, ObjectStorage, Renderer, Vertex};
use blue_engine::uniform_type::Array4;

use crate::tetris_block::TetrisBlock;

pub fn render_blocks(blocks: Vec<&TetrisBlock>, objects: &mut ObjectStorage, renderer: &mut Renderer) {
    let mut block_ids_to_render: HashSet<String> = HashSet::with_capacity(blocks.len());

    blocks.iter().for_each(|block| {
        let id_as_string = format!("{}", block.id);

        block_ids_to_render.insert(id_as_string.clone());

        let block_in_object_store = match objects.get_mut(&id_as_string) {
            Some(b) => b,
            None => {
                objects.new_object(
                    id_as_string.clone(),
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
                let new_object = objects.get_mut(&id_as_string).unwrap();
                new_object.uniform_color = Array4 {data: block.color};
                new_object
            }
        };
        block_in_object_store.set_position((-1.0) + (block.x as f32 / 5.0), (-1.0) + (block.y as f32 / 10.0), 0.0);
    });

    if objects.len() != block_ids_to_render.len() {
        objects.retain(|b, _| block_ids_to_render.contains(b));
    }
}