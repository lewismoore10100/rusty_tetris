use crate::tetris_block::TetrisBlock;

pub struct BlockGroup {
    pub blocks: [TetrisBlock; 4]
}

impl BlockGroup {
    fn move_down(&mut self, other_blocks_in_scene: &[TetrisBlock]) -> Result<(),()> {
        let new_positions = [
            self.blocks[0].moved(0, -1),
            self.blocks[1].moved(0, -1),
            self.blocks[2].moved(0, -1),
            self.blocks[3].moved(0, -1)
        ];

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
        let new_positions = [
            self.blocks[0].moved(-1, 0),
            self.blocks[1].moved(-1, 0),
            self.blocks[2].moved(-1, 0),
            self.blocks[3].moved(-1, 0)
        ];

        if self.can_move(other_blocks_in_scene, &new_positions) {
            self.blocks = new_positions;
        }
    }
    fn move_right(&mut self, other_blocks_in_scene: &[TetrisBlock]) {
        let new_positions = [
            self.blocks[0].moved(1, 0),
            self.blocks[1].moved(1, 0),
            self.blocks[2].moved(1, 0),
            self.blocks[3].moved(1, 0)
        ];

        if self.can_move(other_blocks_in_scene, &new_positions) {
            self.blocks = new_positions;
        }
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
