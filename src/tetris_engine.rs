use crate::square::Square;
use crate::tetris_block::TetrisBlock;
pub struct TetrisEngine {
    current_shape: Square
}

impl TetrisEngine {

    pub fn new() -> TetrisEngine {
        TetrisEngine{current_shape: Square::new()}
    }

    pub fn tick(&mut self){
        self.current_shape.move_down();
    }

    pub fn generate_blocks(&self) -> Vec<TetrisBlock> {
        Vec::from(&self.current_shape.blocks)
    }

}

