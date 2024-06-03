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

    fn rotate(&mut self) {
        let rotated_block = match self.rotation_position {
            N => {
                [
                    self.block_group.blocks.get(0).and_then(|b| Some(TetrisBlock::from_with_new_position(&b, b.x + 2, b.y - 1))).unwrap(),
                    self.block_group.blocks.get(1).and_then(|b| Some(TetrisBlock::from_with_new_position(&b, b.x + 1, b.y))).unwrap(),
                    self.block_group.blocks.get(2).and_then(|b| Some(TetrisBlock::from_with_new_position(&b, b.x, b.y + 1))).unwrap(),
                    self.block_group.blocks.get(3).and_then(|b| Some(TetrisBlock::from_with_new_position(&b, b.x - 1, b.y))).unwrap()
                ]
            }
            E => {
                [
                    self.block_group.blocks.get(0).and_then(|b| Some(TetrisBlock::from_with_new_position(&b, b.x - 1, b.y - 2))).unwrap(),
                    self.block_group.blocks.get(1).and_then(|b| Some(TetrisBlock::from_with_new_position(&b, b.x , b.y - 1))).unwrap(),
                    self.block_group.blocks.get(2).and_then(|b| Some(TetrisBlock::from_with_new_position(&b, b.x + 1, b.y))).unwrap(),
                    self.block_group.blocks.get(3).and_then(|b| Some(TetrisBlock::from_with_new_position(&b, b.x, b.y + 1))).unwrap()
            ]}
            S => {
                [
                    self.block_group.blocks.get(0).and_then(|b| Some(TetrisBlock::from_with_new_position(&b, b.x - 2, b.y + 1))).unwrap(),
                    self.block_group.blocks.get(1).and_then(|b| Some(TetrisBlock::from_with_new_position(&b, b.x - 1 , b.y))).unwrap(),
                    self.block_group.blocks.get(2).and_then(|b| Some(TetrisBlock::from_with_new_position(&b, b.x , b.y - 1))).unwrap(),
                    self.block_group.blocks.get(3).and_then(|b| Some(TetrisBlock::from_with_new_position(&b, b.x + 1, b.y))).unwrap()
                ]
            }
            W => {
                [
                    self.block_group.blocks.get(0).and_then(|b| Some(TetrisBlock::from_with_new_position(&b, b.x + 1, b.y + 2))).unwrap(),
                    self.block_group.blocks.get(1).and_then(|b| Some(TetrisBlock::from_with_new_position(&b, b.x , b.y + 1))).unwrap(),
                    self.block_group.blocks.get(2).and_then(|b| Some(TetrisBlock::from_with_new_position(&b, b.x - 1 , b.y))).unwrap(),
                    self.block_group.blocks.get(3).and_then(|b| Some(TetrisBlock::from_with_new_position(&b, b.x, b.y - 1))).unwrap()
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