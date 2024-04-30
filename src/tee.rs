use RotationPosition::{E, N, S, W};
use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::TetrisBlock;

enum RotationPosition {
    N,E,S,W
}

pub struct Tee {
    pub block_group: BlockGroup,
    pub rotation_position: RotationPosition
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
            rotation_position: N
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
        match self.rotation_position {
            N => {
                self.block_group.blocks.get_mut(0).and_then(|b|{b.x += 1; b.y -= 1; Some(b)});
                self.block_group.blocks.get_mut(1).and_then(|b|{b.x += 1; b.y += 1; Some(b)});
                self.block_group.blocks.get_mut(3).and_then(|b|{b.x -= 1; b.y -= 1; Some(b)});
                self.rotation_position = E;
            }
            E => {
                self.block_group.blocks.get_mut(0).and_then(|b|{b.x -= 1; b.y -= 1; Some(b)});
                self.block_group.blocks.get_mut(1).and_then(|b|{b.x += 1; b.y -= 1; Some(b)});
                self.block_group.blocks.get_mut(3).and_then(|b|{b.x -= 1; b.y += 1; Some(b)});
                self.rotation_position = S;
            }
            S => {
                self.block_group.blocks.get_mut(0).and_then(|b|{b.x -= 1; b.y += 1; Some(b)});
                self.block_group.blocks.get_mut(1).and_then(|b|{b.x -= 1; b.y += 1; Some(b)});
                self.block_group.blocks.get_mut(3).and_then(|b|{b.x += 1; b.y -= 1; Some(b)});
                self.rotation_position = W;
            }
            W => {}
        }
    }
}