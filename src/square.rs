use crate::tetris_block::TetrisBlock;

pub struct Square {
    pub blocks: [TetrisBlock; 4]
}

impl Square {
    pub(crate) fn new() -> Square {
        Square{blocks: [
            TetrisBlock{x: 4, y: 19},
            TetrisBlock{x: 5, y: 19},
            TetrisBlock{x: 4, y: 18},
            TetrisBlock{x: 5, y: 18}
        ]}
    }

    pub fn move_down(&mut self){
        self.blocks.iter_mut().for_each(|b| b.y -= 1)
    }
}