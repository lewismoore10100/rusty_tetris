use crate::rotation_position::RotationPosition;
use crate::rotation_position::RotationPosition::N;
use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::TetrisBlock;

pub struct S {
    pub block_group: BlockGroup,
    pub rotation_position: RotationPosition,
}

impl S {
    pub fn new() -> S {
        S {
            block_group: BlockGroup {
                blocks: [
                    TetrisBlock::new_with_color(4, 19, [1f32, 0f32, 1f32, 1f32]),
                    TetrisBlock::new_with_color(5, 19, [1f32, 0f32, 1f32, 1f32]),
                    TetrisBlock::new_with_color(3, 18, [1f32, 0f32, 1f32, 1f32]),
                    TetrisBlock::new_with_color(4, 18, [1f32, 0f32, 1f32, 1f32]),
                ]
            },
            rotation_position: N,
        }
    }
}

impl PlayableShape for S {

    fn move_down(&mut self, other_blocks_in_scene: &[TetrisBlock])-> Result<(),()> {
        let new_position = self.blocks().move_down(other_blocks_in_scene)?;
        self.set_blocks(new_position);
        Ok(())
    }
    fn move_left(&mut self, other_blocks_in_scene: &[TetrisBlock]) -> Result<(),()> {
        let new_position = self.blocks().move_left(other_blocks_in_scene)?;
        self.set_blocks(new_position);
        Ok(())
    }
    fn move_right(&mut self, other_blocks_in_scene: &[TetrisBlock]) -> Result<(),()> {
        let new_position = self.blocks().move_right(other_blocks_in_scene)?;
        self.set_blocks(new_position);
        Ok(())
    }
    fn blocks(&mut self) -> &mut BlockGroup {
        &mut self.block_group
    }

    fn set_blocks(&mut self, blocks: BlockGroup) {
        self.block_group = blocks;
    }

    fn rotate(&mut self) {
    }
}