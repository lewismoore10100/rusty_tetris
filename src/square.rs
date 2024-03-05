use crate::tetris_block::TetrisBlock;

pub struct Square {
    pub blocks: [TetrisBlock; 4]
}

impl Square {
    pub(crate) fn new() -> Square {
        Square{blocks: [
            TetrisBlock::new(4, 19),
            TetrisBlock::new(5, 19),
            TetrisBlock::new(4, 18),
            TetrisBlock::new(5, 18)
        ]}
    }

    pub fn move_down(&mut self){
        self.blocks.iter_mut().for_each(|b| b.y -= 1)
    }
}