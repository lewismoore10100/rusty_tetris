use crate::direction::Direction;
use crate::rotation_position::RotationPosition;
use crate::rotation_position::RotationPosition::N;
use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::TetrisBlock;

pub struct S {
    pub block_group: BlockGroup,
    pub rotation_position: RotationPosition,
}

impl S {
    pub fn new() -> S {
        S {
            block_group: BlockGroup {
                blocks: [
                    TetrisBlock::new_with_color(4, 19, [1f32, 0f32, 1f32, 1f32]),
                    TetrisBlock::new_with_color(5, 19, [1f32, 0f32, 1f32, 1f32]),
                    TetrisBlock::new_with_color(3, 18, [1f32, 0f32, 1f32, 1f32]),
                    TetrisBlock::new_with_color(4, 18, [1f32, 0f32, 1f32, 1f32]),
                ]
            },
            rotation_position: N,
        }
    }
}

impl PlayableShape for S {

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