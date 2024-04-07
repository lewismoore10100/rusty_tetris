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
        if self.has_collided(self.current_shape.collidable_blocks()) {
            self.current_shape.drain_to(&mut self.merged_blocks);
            self.current_shape = Square::new();
        }
        else {
            self.current_shape.move_down();
        }

    }

    fn has_collided(&self, blocks_to_test: &[TetrisBlock]) -> bool {
        for block_to_test in blocks_to_test {
            if block_to_test.y == 0 {
                return true
            }

            for merged_block in &self.merged_blocks {
                if block_to_test.x == merged_block.x && block_to_test.y-1 == merged_block.y {
                    return true
                }
            }
        }
        false
    }

    pub fn generate_blocks(&self) -> Vec<&TetrisBlock> {
        [self.current_shape.blocks.iter().collect::<Vec<&TetrisBlock>>(), self.merged_blocks.iter().collect::<Vec<&TetrisBlock >>()].concat()
    }

}

