use crate::direction::Direction;
use crate::tetris_block::TetrisBlock;

pub mod l;
pub mod square;
pub mod t;
pub mod s;
pub mod z;
pub mod i;

#[derive(Clone)]
pub struct BlockGroup {
    pub blocks: [TetrisBlock; 4],
}

impl BlockGroup {
    fn move_direction(&self, direction: Direction) -> BlockGroup {
        BlockGroup {
            blocks: match direction {
                Direction::RIGHT => { self.all_moved(1, 0) }
                Direction::LEFT => { self.all_moved(-1, 0) }
                Direction::DOWN => { self.all_moved(0, -1) }
            }
        }
    }
    fn all_moved(&self, x_change: i32, y_change: i32) -> [TetrisBlock; 4] {
        [
            self.blocks[0].moved(x_change, y_change),
            self.blocks[1].moved(x_change, y_change),
            self.blocks[2].moved(x_change, y_change),
            self.blocks[3].moved(x_change, y_change)
        ]
    }
}

pub trait PlayableShape {
    fn move_direction(&self, direction: Direction) -> Box<dyn PlayableShape>;
    fn rotate(&self) -> Box<dyn PlayableShape>;
    fn blocks(&self) -> &[TetrisBlock; 4];
}
