use crate::tetris_block::TetrisBlock;

pub struct TetrisEngine {

}

impl TetrisEngine {

    pub fn new() -> TetrisEngine {
        TetrisEngine{}
    }

    pub fn generate_blocks(&self) -> Vec<TetrisBlock> {
        vec![TetrisBlock{x: 4, y: 19},
             TetrisBlock{x: 5, y: 19},
             TetrisBlock{x: 4, y: 18},
             TetrisBlock{x: 5, y: 18},
        ]
    }

}

