use crate::tetris_block::TetrisBlock;

pub struct TetrisEngine {
    current_shape: Vec<TetrisBlock>
}

impl TetrisEngine {

    pub fn new() -> TetrisEngine {
        TetrisEngine{current_shape: vec![TetrisBlock{x: 4, y: 19},
                                         TetrisBlock{x: 5, y: 19},
                                         TetrisBlock{x: 4, y: 18},
                                         TetrisBlock{x: 5, y: 18},
        ]}
    }

    pub fn tick(&mut self){
        self.current_shape.iter_mut().for_each(|b| b.y -= 1)
    }

    pub fn generate_blocks(&self) -> &Vec<TetrisBlock> {
        &self.current_shape
    }

}

