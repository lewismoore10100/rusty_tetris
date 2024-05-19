use blue_engine::{ObjectSettings, ObjectStorage, Renderer, Vertex};
use blue_engine::uniform_type::Array4;

use crate::tetris_block::TetrisBlock;

pub fn render_blocks(blocks: Vec<&TetrisBlock>, objects: &mut ObjectStorage, renderer: &mut Renderer) {



    blocks.iter().for_each(|block| {

        let block_in_object_store = match objects.get_mut(&block.id) {
            Some(b) => b,
            None => {
                objects.new_object(
                    block.id.clone(),
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
                let new_object = objects.get_mut(&block.id).unwrap();
                new_object.uniform_color = Array4 {data: block.color};
                new_object
            }
        };


        let new_x = (-1.0) + (block.x as f32 / 5.0);
        let new_y = (-1.0) + (block.y as f32 / 10.0);

        //let start_time = Instant::now();

        if block_in_object_store.position.x != new_x || block_in_object_store.position.y != new_y {
            block_in_object_store.set_position(new_x, new_y, 0.0);
        }
        //println!("{}", start_time.elapsed().as_micros());
    });

    if objects.len() != blocks.len() {
        objects.retain(|b, _| blocks.iter().any(|b2| b2.id.eq(b)));
    }


}