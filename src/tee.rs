use RotationPosition::{E, N, S, W};
use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::TetrisBlock;

enum RotationPosition {
    N,
    E,
    S,
    W,
}

pub struct Tee {
    pub block_group: BlockGroup,
    pub rotation_position: RotationPosition,
}

impl Tee {
    pub fn new() -> Tee {
        Tee {
            block_group: BlockGroup {
                blocks: vec![
                    TetrisBlock::new(5, 19),
                    TetrisBlock::new(4, 18),
                    TetrisBlock::new(5, 18),
                    TetrisBlock::new(6, 18),
                ]
            },
            rotation_position: N,
        }
    }
}

impl PlayableShape for Tee {
    fn block_group_mut(&mut self) -> &mut BlockGroup {
        &mut self.block_group
    }

    fn block_group(&self) -> &BlockGroup {
        &self.block_group
    }

    fn rotate(&mut self) {
        let rotated_block = match self.rotation_position {
            N => {
                vec![
                    self.block_group.blocks.get(0).and_then(|b| Some(TetrisBlock::new(b.x + 1, b.y - 1))).unwrap(),
                    self.block_group.blocks.get(1).and_then(|b| Some(TetrisBlock::new(b.x + 1, b.y + 1))).unwrap(),
                    self.block_group.blocks.get(2).and_then(|b| Some(TetrisBlock::new(b.x, b.y))).unwrap(),
                    self.block_group.blocks.get(3).and_then(|b| Some(TetrisBlock::new(b.x - 1, b.y - 1))).unwrap()
                ]
            }
            E => {
                vec![
                    self.block_group.blocks.get(0).and_then(|b| Some(TetrisBlock::new(b.x - 1, b.y - 1))).unwrap(),
                    self.block_group.blocks.get(1).and_then(|b| Some(TetrisBlock::new(b.x + 1, b.y - 1))).unwrap(),
                    self.block_group.blocks.get(2).and_then(|b| Some(TetrisBlock::new(b.x, b.y))).unwrap(),
                    self.block_group.blocks.get(3).and_then(|b| Some(TetrisBlock::new(b.x - 1, b.y + 1))).unwrap()
                ]
            }
            S => {
                vec![
                    self.block_group.blocks.get(0).and_then(|b| Some(TetrisBlock::new(b.x - 1, b.y + 1))).unwrap(),
                    self.block_group.blocks.get(1).and_then(|b| Some(TetrisBlock::new(b.x - 1, b.y + 1))).unwrap(),
                    self.block_group.blocks.get(2).and_then(|b| Some(TetrisBlock::new(b.x, b.y))).unwrap(),
                    self.block_group.blocks.get(3).and_then(|b| Some(TetrisBlock::new(b.x + 1, b.y - 1))).unwrap()
                ]
            }
            W => {
                vec![
                    self.block_group.blocks.get(0).and_then(|b| Some(TetrisBlock::new(b.x + 1, b.y + 1))).unwrap(),
                    self.block_group.blocks.get(1).and_then(|b| Some(TetrisBlock::new(b.x - 1, b.y - 1))).unwrap(),
                    self.block_group.blocks.get(2).and_then(|b| Some(TetrisBlock::new(b.x, b.y))).unwrap(),
                    self.block_group.blocks.get(3).and_then(|b| Some(TetrisBlock::new(b.x + 1, b.y + 1))).unwrap()
                ]
            }
        };

        if !rotated_block.iter().any(|b| b.x == 10){
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