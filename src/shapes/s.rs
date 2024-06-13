use crate::direction::Direction;
use crate::rotation_position::RotationPosition;
use crate::rotation_position::RotationPosition::N;
use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::TetrisBlock;

#[derive(Clone)]
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

    fn move_direction(&self, direction: Direction, other_blocks_in_scene: &[TetrisBlock])-> Result<Box<dyn PlayableShape>,()> {
        let new_position = self.block_group.move_direction(direction, other_blocks_in_scene)?;
        Ok(Box::new(S{ block_group: new_position, rotation_position: self.rotation_position.clone()}))
    }

    fn rotate(&self) -> Box<dyn PlayableShape> {
        Box::new(self.clone())
    }

    fn blocks(&self) -> &[TetrisBlock; 4] {
        &self.block_group.blocks
    }
}