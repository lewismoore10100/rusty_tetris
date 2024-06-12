use crate::direction::Direction;
use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::TetrisBlock;

pub struct Square {
    pub block_group: BlockGroup,
}

impl Square {
    pub fn new() -> Square {
        Square {
            block_group: BlockGroup {
                blocks: [
                    TetrisBlock::new_with_color(4, 19, [0f32, 0f32, 1f32, 1f32]),
                    TetrisBlock::new_with_color(5, 19, [0f32, 0f32, 1f32, 1f32]),
                    TetrisBlock::new_with_color(4, 18, [0f32, 0f32, 1f32, 1f32]),
                    TetrisBlock::new_with_color(5, 18, [0f32, 0f32, 1f32, 1f32]),
                ]
            }
        }
    }
}

impl PlayableShape for Square {

    fn move_direction(&mut self, direction: Direction, other_blocks_in_scene: &[TetrisBlock])-> Result<(),()> {
        let new_position = self.block_group.move_direction(direction, other_blocks_in_scene)?;
        self.block_group = new_position;
        Ok(())
    }

    fn rotate(&mut self) {
    }

    fn blocks(&self) -> &BlockGroup {
        &self.block_group
    }
}