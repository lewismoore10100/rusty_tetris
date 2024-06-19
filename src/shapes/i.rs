use crate::direction::Direction;
use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::TetrisBlock;

#[derive(Clone)]
pub struct I {
    pub block_group: BlockGroup,
    pub rotated: bool
}

impl I {
    pub fn new() -> I {
        I {
            block_group: BlockGroup {
                blocks: [
                    TetrisBlock::new_with_color(4, 19, [1f32, 1f32, 0f32, 1f32]),
                    TetrisBlock::new_with_color(4, 18, [1f32, 1f32, 0f32, 1f32]),
                    TetrisBlock::new_with_color(4, 17, [1f32, 1f32, 0f32, 1f32]),
                    TetrisBlock::new_with_color(4, 16, [1f32, 1f32, 0f32, 1f32]),
                ]
            },
            rotated: false,
        }
    }
}

impl PlayableShape for I {
    fn move_direction(&self, direction: Direction) -> Box<dyn PlayableShape> {
        let new_position = self.block_group.move_direction(direction);
        Box::new(I { block_group: new_position, rotated: self.rotated })
    }

    fn rotate(&self) -> Box<dyn PlayableShape> {

        Box::new(I {
            block_group: BlockGroup {
                blocks: self.block_group.blocks.clone()
            },
            rotated: !self.rotated
        })
    }

    fn blocks(&self) -> &[TetrisBlock; 4] {
        &self.block_group.blocks
    }
}