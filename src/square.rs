use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::TetrisBlock;

pub struct Square {
    pub block_group: BlockGroup,
}

impl Square {
    pub(crate) fn new() -> Square {
        Square {
            block_group: BlockGroup {
                blocks: vec![
                    TetrisBlock::new(4, 19),
                    TetrisBlock::new(5, 19),
                    TetrisBlock::new(4, 18),
                    TetrisBlock::new(5, 18),
                ]
            }
        }
    }
}

impl PlayableShape for Square {
    fn block_group_mut(&mut self) -> &mut BlockGroup {
        &mut self.block_group
    }

    fn block_group(&self) -> &BlockGroup {
        &self.block_group
    }
}