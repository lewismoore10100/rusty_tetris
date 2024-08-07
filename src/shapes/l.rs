use RotationPosition::{E, S, W};

use crate::direction::Direction;
use crate::rotation_position::RotationPosition;
use crate::rotation_position::RotationPosition::N;
use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::{COLOR, TetrisBlock};

pub struct L {
    pub block_group: BlockGroup,
    pub rotation_position: RotationPosition,
}

impl L {
    pub fn new() -> L {
        L {
            block_group: BlockGroup {
                blocks: [
                    TetrisBlock::new_with_color(4, 19, COLOR::ORANGE),
                    TetrisBlock::new_with_color(4, 18, COLOR::ORANGE),
                    TetrisBlock::new_with_color(4, 17, COLOR::ORANGE),
                    TetrisBlock::new_with_color(5, 17, COLOR::ORANGE),
                ]
            },
            rotation_position: N,
        }
    }
}

impl PlayableShape for L {
    fn move_direction(&self, direction: Direction) -> Box<dyn PlayableShape> {
        let new_position = self.block_group.move_direction(direction);
        Box::new(L { block_group: new_position, rotation_position: self.rotation_position.clone() })
    }

    fn rotate(&self) -> Box<dyn PlayableShape> {
        let rotated_block = match self.rotation_position {
            N => {
                [
                    self.block_group.blocks[0].moved(2, -1),
                    self.block_group.blocks[1].moved(1, 0),
                    self.block_group.blocks[2].moved(0, 1),
                    self.block_group.blocks[3].moved(-1, 0)
                ]
            }
            E => {
                [
                    self.block_group.blocks[0].moved(-1, -2),
                    self.block_group.blocks[1].moved(0, -1),
                    self.block_group.blocks[2].moved(1, 0),
                    self.block_group.blocks[3].moved(0, 1),
                ]
            }
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
                    self.block_group.blocks[2].moved(-1, 0),
                    self.block_group.blocks[3].moved(0, -1)
                ]
            }
        };

        Box::new(L {
            block_group: BlockGroup {
                blocks: rotated_block
            },
            rotation_position: self.rotation_position.next_position(),
        })
    }

    fn blocks(&self) -> &[TetrisBlock; 4] {
        &self.block_group.blocks
    }
}