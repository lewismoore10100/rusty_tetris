use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::TetrisBlock;

pub struct Tee {
    pub block_group: BlockGroup,
}

impl Tee {
    pub(crate) fn new() -> Tee {
        Tee {
            block_group: BlockGroup {
                blocks: vec![
                    TetrisBlock::new(4, 19),
                    TetrisBlock::new(5, 18),
                    TetrisBlock::new(5, 19),
                    TetrisBlock::new(5, 20),
                ]
            }
        }
    }
}

impl PlayableShape for Tee {
    fn block_group_mut(&mut self) -> &mut BlockGroup {
        &mut self.block_group
    }

    fn block_group(&self) -> &BlockGroup {
        &self.block_group
    }
}