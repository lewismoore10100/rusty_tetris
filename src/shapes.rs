use crate::tetris_block::TetrisBlock;
use crate::direction::Direction;

pub mod l;
pub mod square;
pub mod t;
pub mod s;

pub struct BlockGroup {
    pub blocks: [TetrisBlock; 4]
}

impl BlockGroup {

    fn move_direction(&self, direction: Direction, other_blocks_in_scene: &[TetrisBlock]) -> Result<BlockGroup,()> {
        let new_positions = match direction {
            Direction::RIGHT => {self.all_moved(1, 0)}
            Direction::LEFT => {self.all_moved(-1, 0)}
            Direction::DOWN => {self.all_moved(0, -1)}
        };

        if self.can_move(other_blocks_in_scene, &new_positions) {
            Ok(BlockGroup { blocks: new_positions})
        }
        else
        {
            Err(())
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

    fn can_move(&self, other_blocks_in_scene: &[TetrisBlock], new_position: &[TetrisBlock]) -> bool {
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

            for block_in_scene in other_blocks_in_scene {
                if current_block_to_move.x == block_in_scene.x &&
                    current_block_to_move.y == block_in_scene.y {
                    return false;
                }
            }
        }
        true
    }
}

pub trait PlayableShape {
    fn move_direction(&mut self, direction: Direction, other_blocks_in_scene: &[TetrisBlock]) -> Result<(),()>;
    fn rotate(&mut self);
    fn blocks(&self) -> &[TetrisBlock; 4];
}
