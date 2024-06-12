use RotationPosition::{E, S, W};
use crate::rotation_position::RotationPosition;
use crate::rotation_position::RotationPosition::N;
use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::TetrisBlock;

pub struct L {
    pub block_group: BlockGroup,
    pub rotation_position: RotationPosition,
}

impl L {
    pub fn new() -> L {
        L {
            block_group: BlockGroup {
                blocks: [
                    TetrisBlock::new_with_color(4, 19, [1f32, 0f32, 0f32, 1f32]),
                    TetrisBlock::new_with_color(4, 18, [1f32, 0f32, 0f32, 1f32]),
                    TetrisBlock::new_with_color(4, 17, [1f32, 0f32, 0f32, 1f32]),
                    TetrisBlock::new_with_color(5, 17, [1f32, 0f32, 0f32, 1f32]),
                ]
            },
            rotation_position: N,
        }
    }
}

impl PlayableShape for L {
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
                    self.block_group.blocks[0].moved(2, -1),
                    self.block_group.blocks[1].moved(1, 0),
                    self.block_group.blocks[2].moved(0,1),
                    self.block_group.blocks[3].moved(-1, 0)
                ]
            }
            E => {
                [
                    self.block_group.blocks[0].moved(-1, -2),
                    self.block_group.blocks[1].moved(0, -1),
                    self.block_group.blocks[2].moved(1, 0),
                    self.block_group.blocks[3].moved(0, 1),
            ]}
            S => {
                [
                    self.block_group.blocks[0].moved(-2, 1),
                    self.block_group.blocks[1].moved(-1, 0),
                    self.block_group.blocks[2].moved(0, -1),
                    self.block_group.blocks[3].moved(1, 0)
                ]
            }
            W => {
                [
                    self.block_group.blocks[0].moved(1, 2),
                    self.block_group.blocks[1].moved(0, 1),
                    self.block_group.blocks[2].moved(-1 , 0),
                    self.block_group.blocks[3].moved(0, -1)
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