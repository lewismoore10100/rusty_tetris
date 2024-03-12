use crate::square::Square;
use crate::tetris_block::TetrisBlock;
pub struct TetrisEngine {
    current_shape: Square,
    merged_blocks: Vec<TetrisBlock>
}

impl TetrisEngine {

    pub fn new() -> TetrisEngine {
        TetrisEngine{current_shape: Square::new(), merged_blocks: vec![] }
    }

    pub fn tick(&mut self){
        if self.current_shape.hit() {
            self.current_shape.drain_to(&mut self.merged_blocks);
            self.current_shape = Square::new();
        }
        self.current_shape.move_down();
    }

    pub fn generate_blocks(&self) -> Vec<&TetrisBlock> {
        [self.current_shape.blocks.iter().collect::<Vec<&TetrisBlock>>(), self.merged_blocks.iter().collect::<Vec<&TetrisBlock >>()].concat()
    }

}

