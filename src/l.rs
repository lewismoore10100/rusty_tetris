use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::TetrisBlock;

pub struct L {
    pub block_group: BlockGroup,
}

impl L {
    pub fn new() -> L {
        L {
            block_group: BlockGroup {
                blocks: vec![
                    TetrisBlock::new_with_color(4, 19, [1f32, 0f32, 0f32, 1f32]),
                    TetrisBlock::new_with_color(4, 18, [1f32, 0f32, 0f32, 1f32]),
                    TetrisBlock::new_with_color(4, 17, [1f32, 0f32, 0f32, 1f32]),
                    TetrisBlock::new_with_color(5, 17, [1f32, 0f32, 0f32, 1f32]),
                ]
            }
        }
    }
}

impl PlayableShape for L {
    fn block_group_mut(&mut self) -> &mut BlockGroup {
        &mut self.block_group
    }

    fn block_group(&self) -> &BlockGroup {
        &self.block_group
    }
}