use std::collections::HashMap;
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

    pub fn with_initial_state(initial_state: Vec<TetrisBlock>) -> TetrisEngine {
        TetrisEngine{current_shape: Square::new(), merged_blocks: initial_state }
    }

    pub fn tick(&mut self){
        if self.has_collided(self.current_shape.collidable_blocks()) {
            self.current_shape.drain_to(&mut self.merged_blocks);
            self.remove_completed_rows();
            self.current_shape = Square::new();
        }
        else {
            self.current_shape.move_down();
        }

    }

    pub fn move_left(&mut self) {
        self.current_shape.move_left();
    }

    pub fn move_right(&mut self) {
        self.current_shape.move_right();
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

    fn remove_completed_rows(&mut self){
        println!("{}", "removing completed rows");
        if self.merged_blocks.len() < 10 {
            return;
        }

        let mut block_count_per_row : [u32;20] = [0;20];

        self.merged_blocks.iter().for_each(|block| {
            block_count_per_row[block.y as usize] = block_count_per_row[block.y as usize]+1
        });

        self.merged_blocks.retain(|block|{ block_count_per_row[block.y as usize] != 10});

        let rows_removed = block_count_per_row.iter().filter(|&&count| count == 10).count() as u32;

        self.merged_blocks.iter_mut().for_each(|mut block| {
            block.y = block.y - rows_removed;
        });

    }

    pub fn generate_blocks(&self) -> Vec<&TetrisBlock> {
        [self.current_shape.blocks.iter().collect::<Vec<&TetrisBlock>>(), self.merged_blocks.iter().collect::<Vec<&TetrisBlock >>()].concat()
    }

}

