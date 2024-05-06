use crate::tetris_block::TetrisBlock;

pub struct BlockGroup {
    pub blocks: Vec<TetrisBlock>,
}

impl BlockGroup {
    fn drain_to(&mut self, merge_to: &mut Vec<TetrisBlock>) {
        merge_to.append(&mut self.blocks);
    }

    fn move_down(&mut self, other_blocks_in_scene: &[TetrisBlock]) -> Result<(),()> {
        if self.can_move(other_blocks_in_scene, 0, -1) {
            self.blocks.iter_mut().for_each(|b| b.y -= 1);
            return Ok(());
        }
        Err(())
    }

    fn move_left(&mut self, other_blocks_in_scene: &[TetrisBlock]) {
        if self.can_move(other_blocks_in_scene, -1, 0) {
            self.blocks.iter_mut().for_each(|b| b.x -= 1)
        }

    }
    fn move_right(&mut self, other_blocks_in_scene: &[TetrisBlock]) {
        if self.can_move(other_blocks_in_scene, 1, 0) {
            self.blocks.iter_mut().for_each(|b| b.x += 1)
        }
    }

    fn can_move(&self, other_blocks_in_scene: &[TetrisBlock], x_movement: i32, y_movement: i32) -> bool {
        for current_block_to_move in &self.blocks {
            if current_block_to_move.y + y_movement == -1 {
                return false;
            }
            if current_block_to_move.x + x_movement == -1 {
                return false;
            }
            if current_block_to_move.x + x_movement == 10 {
                return false;
            }

            for block_in_scene in other_blocks_in_scene {
                if current_block_to_move.x + x_movement == block_in_scene.x &&
                    current_block_to_move.y + y_movement == block_in_scene.y {
                    return false;
                }
            }
        }
        true
    }
}

pub trait PlayableShape {
    fn blocks(&self) -> &[TetrisBlock] {
        &self.block_group().blocks[..]
    }
    fn drain_to(&mut self, merge_to: &mut Vec<TetrisBlock>) {
        self.block_group_mut().drain_to(merge_to)
    }
    fn move_down(&mut self, other_blocks_in_scene: &[TetrisBlock]) -> Result<(),()> {
        self.block_group_mut().move_down(other_blocks_in_scene)
    }
    fn move_left(&mut self, other_blocks_in_scene: &[TetrisBlock]) {
        self.block_group_mut().move_left(other_blocks_in_scene)
    }
    fn move_right(&mut self, other_blocks_in_scene: &[TetrisBlock]) {
        self.block_group_mut().move_right(other_blocks_in_scene)
    }
    fn block_group_mut(&mut self) -> &mut BlockGroup;
    fn block_group(&self) -> &BlockGroup;
    fn rotate(&mut self){}
}
