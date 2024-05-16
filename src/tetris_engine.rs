use crate::shapes::{PlayableShape};
use crate::square::Square;
use crate::tee::Tee;
use crate::tetris_block::TetrisBlock;
extern crate rand;
use rand::Rng;

pub struct TetrisEngine {
    current_shape: Box<dyn PlayableShape>,
    merged_blocks: Vec<TetrisBlock>,
    generate_next_shape: fn() -> Box<dyn PlayableShape>
}

fn random_shape_generator() -> Box<dyn PlayableShape>{
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..2);

    match random_number {
        0 => Box::new(Square::new()),
        1 => Box::new(Tee::new()),
        _ => Box::new(Square::new())
    }
}

impl TetrisEngine {
    pub fn new() -> TetrisEngine {
        TetrisEngine {
            current_shape: Box::new(Square::new()),
            merged_blocks: vec![],
            generate_next_shape: random_shape_generator
        }
    }

    pub fn with_initial_state(initial_state: Vec<TetrisBlock>, shape_generator: fn() -> Box<dyn PlayableShape>) -> TetrisEngine {
        TetrisEngine { current_shape: shape_generator(), merged_blocks: initial_state, generate_next_shape: shape_generator }
    }

    pub fn tick(&mut self) {
        if self.current_shape.move_down(self.merged_blocks.as_slice()).is_err() {
            self.current_shape.drain_to(&mut self.merged_blocks);
            self.remove_completed_rows();
            self.current_shape = (self.generate_next_shape)()
        }
    }

    pub fn move_left(&mut self) {
        self.current_shape.move_left(self.merged_blocks.as_slice());
    }

    pub fn move_right(&mut self) {
        self.current_shape.move_right(self.merged_blocks.as_slice());
    }

    pub fn drop(&mut self) {
        loop {
            if self.current_shape.move_down(self.merged_blocks.as_slice()).is_err() {
                self.tick();
                break;
            }
        }
    }

    pub fn rotate(&mut self) {
        self.current_shape.rotate();
    }

    fn remove_completed_rows(&mut self) {
        if self.merged_blocks.len() < 10 {
            return;
        }
        self.remove_completed_rows_starting_from(0);
    }

    fn remove_completed_rows_starting_from(&mut self, row: i32){
        if row > 19 {
            return;
        }

        if self.merged_blocks.iter().filter(|b| b.y == row).count() == 10 {
            self.merged_blocks.retain(|b| { b.y != row});
            self.merged_blocks.iter_mut().for_each(|b| { if b.y > row {b.y -= 1}});
            self.remove_completed_rows_starting_from(row);
        }
        else {
            self.remove_completed_rows_starting_from(row+1)
        }
    }

    pub fn blocks_for_rendering(&self) -> Vec<&TetrisBlock> {
        let mut all_blocks: Vec<&TetrisBlock> = Vec::with_capacity(self.current_shape.blocks().len() + self.merged_blocks.len());
        self.current_shape.blocks().iter().for_each(|b| all_blocks.push(b));
        self.merged_blocks.iter().for_each(|b| all_blocks.push(b));
        all_blocks
    }
}

