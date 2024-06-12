use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::TetrisBlock;

pub struct Square {
    pub block_group: BlockGroup,
}

impl Square {
    pub fn new() -> Square {
        Square {
            block_group: BlockGroup {
                blocks: [
                    TetrisBlock::new_with_color(4, 19, [0f32, 0f32, 1f32, 1f32]),
                    TetrisBlock::new_with_color(5, 19, [0f32, 0f32, 1f32, 1f32]),
                    TetrisBlock::new_with_color(4, 18, [0f32, 0f32, 1f32, 1f32]),
                    TetrisBlock::new_with_color(5, 18, [0f32, 0f32, 1f32, 1f32]),
                ]
            }
        }
    }
}

impl PlayableShape for Square {

    fn move_down(&mut self, other_blocks_in_scene: &[TetrisBlock])-> Result<(),()> {
        let new_position = self.block_group.move_down(other_blocks_in_scene)?;
        self.block_group = new_position;
        Ok(())
    }
    fn move_left(&mut self, other_blocks_in_scene: &[TetrisBlock]) -> Result<(),()> {
        let new_position = self.block_group.move_left(other_blocks_in_scene)?;
        self.block_group = new_position;
        Ok(())
    }
    fn move_right(&mut self, other_blocks_in_scene: &[TetrisBlock]) -> Result<(),()> {
        let new_position = self.block_group.move_right(other_blocks_in_scene)?;
        self.block_group = new_position;
        Ok(())
    }

    fn rotate(&mut self) {
    }

    fn blocks(&self) -> &BlockGroup {
        &self.block_group
    }
}