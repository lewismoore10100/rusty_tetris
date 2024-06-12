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
    fn blocks(&mut self) -> &mut BlockGroup {
        &mut self.block_group
    }

    fn set_blocks(&mut self, blocks: BlockGroup) {
        self.block_group = blocks;
    }
}