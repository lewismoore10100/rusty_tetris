use crate::rotation_position::RotationPosition;
use crate::rotation_position::RotationPosition::{E, N, S, W};
use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::TetrisBlock;

pub struct T {
    pub block_group: BlockGroup,
    pub rotation_position: RotationPosition,
}

impl T {
    pub fn new() -> T {
        T {
            block_group: BlockGroup {
                blocks: [
                    TetrisBlock::new_with_color(5, 19, [0f32, 1f32, 0f32, 1f32]),
                    TetrisBlock::new_with_color(4, 18, [0f32, 1f32, 0f32, 1f32]),
                    TetrisBlock::new_with_color(5, 18, [0f32, 1f32, 0f32, 1f32]),
                    TetrisBlock::new_with_color(6, 18, [0f32, 1f32, 0f32, 1f32]),
                ]
            },
            rotation_position: N,
        }
    }
}

impl PlayableShape for T {

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
        let rotated_block = match self.rotation_position {
            N => {
                [
                    self.block_group.blocks[0].moved(1, -1),
                    self.block_group.blocks[1].moved(1, 1),
                    self.block_group.blocks[2].moved(0, 0),
                    self.block_group.blocks[3].moved(-1, -1)
                ]
            }
            E => {
                [
                    self.block_group.blocks[0].moved(-1, -1),
                    self.block_group.blocks[1].moved(1, -1),
                    self.block_group.blocks[2].moved(0, 0),
                    self.block_group.blocks[3].moved(-1, 1)
                ]
            }
            S => {
                [
                    self.block_group.blocks[0].moved(-1, 1),
                    self.block_group.blocks[1].moved(-1, 1),
                    self.block_group.blocks[2].moved(0, 0),
                    self.block_group.blocks[3].moved(1, -1)
                ]
            }
            W => {
                [
                    self.block_group.blocks[0].moved(1, 1),
                    self.block_group.blocks[1].moved(-1, -1),
                    self.block_group.blocks[2].moved(0, 0),
                    self.block_group.blocks[3].moved(1, 1)
                ]
            }
        };

        if !rotated_block.iter().any(|b| b.x == 10 || b.x == -1){
            self.block_group = BlockGroup{blocks: rotated_block};
            self.rotation_position = match self.rotation_position {
                N => {E}
                E => {S}
                S => {W}
                W => {N}
            }
        }
    }
}