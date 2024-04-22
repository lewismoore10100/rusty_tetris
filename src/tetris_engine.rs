use crate::shapes::{PlayableShape};
use crate::square::Square;
use crate::tetris_block::TetrisBlock;

pub struct TetrisEngine {
    current_shape: Box<dyn PlayableShape>,
    merged_blocks: Vec<TetrisBlock>,
}

impl TetrisEngine {
    pub fn new() -> TetrisEngine {
        TetrisEngine { current_shape: Box::new(Square::new()), merged_blocks: vec![] }
    }

    pub fn with_initial_state(initial_state: Vec<TetrisBlock>) -> TetrisEngine {
        TetrisEngine { current_shape: Box::new(Square::new()), merged_blocks: initial_state }
    }

    pub fn tick(&mut self) {
        if !self.can_move_down(self.current_shape.blocks()) {
            self.current_shape.drain_to(&mut self.merged_blocks);
            self.remove_completed_rows();
            self.current_shape = Box::new(Square::new());
        } else {
            self.current_shape.move_down();
        }
    }

    pub fn move_left(&mut self) {
        if self.can_move_left(self.current_shape.blocks()) {
            self.current_shape.move_left();
        }
    }

    pub fn move_right(&mut self) {
        if self.can_move_right(self.current_shape.blocks()) {
            self.current_shape.move_right();
        }
    }

    pub fn drop(&mut self) {
        loop {
            self.current_shape.move_down();
            if !self.can_move_down(self.current_shape.blocks()) {
                self.tick();
                break;
            }
        }
    }

    fn can_move_right(&self, blocks_to_test: &[TetrisBlock]) -> bool {
        self.can_move(blocks_to_test, 1, 0)
    }

    fn can_move_left(&self, blocks_to_test: &[TetrisBlock]) -> bool {
        self.can_move(blocks_to_test, -1, 0)
    }

    fn can_move_down(&self, blocks_to_test: &[TetrisBlock]) -> bool {
        self.can_move(blocks_to_test, 0, -1)
    }

    fn can_move(&self, blocks_to_test: &[TetrisBlock], x_movement: i32, y_movement: i32) -> bool {
        for block_to_test in blocks_to_test {
            if block_to_test.y as i32 + y_movement == -1 {
                return false;
            }
            if block_to_test.x as i32 + x_movement == -1 {
                return false;
            }
            if block_to_test.x as i32 + x_movement == 10 {
                return false;
            }

            for merged_block in &self.merged_blocks {
                if block_to_test.x as i32 + x_movement == merged_block.x as i32 && block_to_test.y as i32 + y_movement == merged_block.y as i32 {
                    return false;
                }
            }
        }
        true
    }

    fn remove_completed_rows(&mut self) {
        if self.merged_blocks.len() < 10 {
            return;
        }

        let mut block_count_per_row: [u32; 20] = [0; 20];

        self.merged_blocks.iter().for_each(|block| {
            block_count_per_row[block.y as usize] = block_count_per_row[block.y as usize] + 1
        });

        self.merged_blocks.retain(|block| { block_count_per_row[block.y as usize] != 10 });

        let rows_removed = block_count_per_row.iter().filter(|&&count| count == 10).count() as u32;

        self.merged_blocks.iter_mut().for_each(|mut block| {
            block.y = block.y - rows_removed;
        });
    }

    pub fn blocks_for_rendering(&self) -> Vec<&TetrisBlock> {
        let mut all_blocks: Vec<&TetrisBlock> = Vec::with_capacity(self.current_shape.blocks().len() + self.merged_blocks.len());
        self.current_shape.blocks().iter().for_each(|b| all_blocks.push(b));
        self.merged_blocks.iter().for_each(|b| all_blocks.push(b));
        all_blocks
    }
}

