use crate::tetris_block::TetrisBlock;

pub mod l;
pub mod square;
pub mod tee;

pub struct BlockGroup {
    pub blocks: [TetrisBlock; 4]
}

impl BlockGroup {
    fn move_down(&mut self, other_blocks_in_scene: &[TetrisBlock]) -> Result<(),()> {
        let new_positions = self.all_moved(0, -1);

        if self.can_move(other_blocks_in_scene, &new_positions) {
            self.blocks = new_positions;
            Ok(())
        }
        else
        {
            Err(())
        }
    }

    fn move_left(&mut self, other_blocks_in_scene: &[TetrisBlock]) {
        let new_positions = self.all_moved(-1, 0);

        if self.can_move(other_blocks_in_scene, &new_positions) {
            self.blocks = new_positions;
        }
    }
    fn move_right(&mut self, other_blocks_in_scene: &[TetrisBlock]) {
        let new_positions = self.all_moved(1, 0);

        if self.can_move(other_blocks_in_scene, &new_positions) {
            self.blocks = new_positions;
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
    fn move_down(&mut self, other_blocks_in_scene: &[TetrisBlock]) -> Result<(),()> {
        self.blocks().move_down(other_blocks_in_scene)
    }
    fn move_left(&mut self, other_blocks_in_scene: &[TetrisBlock]) {
        self.blocks().move_left(other_blocks_in_scene)
    }
    fn move_right(&mut self, other_blocks_in_scene: &[TetrisBlock]) {
        self.blocks().move_right(other_blocks_in_scene)
    }
    fn blocks(&mut self) -> &mut BlockGroup;
    fn rotate(&mut self){}
}
