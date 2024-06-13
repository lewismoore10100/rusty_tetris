extern crate rand;

use rand::Rng;
use Direction::RIGHT;
use crate::direction::Direction;
use crate::direction::Direction::{DOWN, LEFT};
use crate::scoring::calculate_score;
use crate::shapes::l::L;
use crate::shapes::{PlayableShape};
use crate::shapes::s::S;
use crate::shapes::square::Square;
use crate::shapes::t::T;
use crate::tetris_block::TetrisBlock;

pub struct TetrisEngine {
    current_shape: Box<dyn PlayableShape>,
    merged_blocks: Vec<TetrisBlock>,
    generate_next_shape: fn() -> Box<dyn PlayableShape>,
    score: u32
}

fn random_shape_generator() -> Box<dyn PlayableShape>{
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..4);

    match random_number {
        0 => Box::new(Square::new()),
        1 => Box::new(T::new()),
        2 => Box::new(L::new()),
        3 => Box::new(S::new()),
        _ => Box::new(Square::new())
    }
}

impl TetrisEngine {
    pub fn new() -> TetrisEngine {
        TetrisEngine {
            current_shape: Box::new(Square::new()),
            merged_blocks: vec![],
            generate_next_shape: random_shape_generator,
            score: 0
        }
    }

    pub fn with_initial_state(initial_state: Vec<TetrisBlock>, shape_generator: fn() -> Box<dyn PlayableShape>) -> TetrisEngine {
        TetrisEngine {
            current_shape: shape_generator(),
            merged_blocks: initial_state,
            generate_next_shape: shape_generator,
            score: 0
        }
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn tick(&mut self) {
        let new_position = self.current_shape.move_direction(DOWN);

        if !self.can_move(new_position.blocks()) {
            self.current_shape.blocks().iter().for_each(|b| {
                let new_b = b.clone();
                self.merged_blocks.push(new_b);
            });
            self.remove_completed_rows();
            self.current_shape = (self.generate_next_shape)()
        }
        else {
            self.current_shape = new_position;
        }
    }

    pub fn move_left(&mut self) {
        let new_position = self.current_shape.move_direction(LEFT);

        if self.can_move(new_position.blocks()){
            self.current_shape = new_position;
        }
    }

    pub fn move_right(&mut self) {
        let new_position = self.current_shape.move_direction(RIGHT);

        if self.can_move(new_position.blocks()){
            self.current_shape = new_position;
        }
    }

    pub fn drop(&mut self) {
        loop {
            let new_position = self.current_shape.move_direction(DOWN);

            if self.can_move(new_position.blocks()){
                self.current_shape = new_position;
            }
            else
            {
                self.tick();
                break;
            }
        }
    }

    fn can_move(&self, new_position: &[TetrisBlock]) -> bool {
        for current_block_to_move in new_position {
            if current_block_to_move.y == -1 {
                return false;
            }
            if current_block_to_move.x == -1 {
                return false;
            }
            if current_block_to_move.x == 10 {
                return false;
            }

            for block_in_scene in &self.merged_blocks {
                if current_block_to_move.x == block_in_scene.x &&
                    current_block_to_move.y == block_in_scene.y {
                    return false;
                }
            }
        }
        true
    }

    pub fn rotate(&mut self) {
        let new_position = self.current_shape.rotate();

        if !new_position.blocks().iter().any(|b| b.x == 10 || b.x == -1){
            self.current_shape = new_position;
        }
    }

    fn remove_completed_rows(&mut self) {
        if self.merged_blocks.len() < 10 {
            return;
        }
        let removed_rows = self.remove_completed_rows_starting_from(0);
        self.score = self.score + calculate_score(removed_rows);
    }

    fn remove_completed_rows_starting_from(&mut self, row: i32) -> u32{
        if row > 19 {
            return 0;
        }

        if self.merged_blocks.iter().filter(|b| b.y == row).count() == 10 {
            self.merged_blocks.retain(|b| { b.y != row});
            self.merged_blocks.iter_mut().for_each(|b| { if b.y > row {b.y -= 1}});
            return self.remove_completed_rows_starting_from(row) + 1;
        }
        else {
            return self.remove_completed_rows_starting_from(row+1);
        }
    }

    pub fn blocks_for_rendering(&mut self) -> Vec<&TetrisBlock> {
        let mut all_blocks: Vec<&TetrisBlock> = Vec::with_capacity(self.current_shape.blocks().len() + self.merged_blocks.len());
        self.current_shape.blocks().iter().for_each(|b| all_blocks.push(b));
        self.merged_blocks.iter().for_each(|b| all_blocks.push(b));
        all_blocks
    }
}

