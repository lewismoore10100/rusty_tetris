use crate::direction::Direction;
use crate::rotation_position::RotationPosition;
use crate::rotation_position::RotationPosition::{E, N, S, W};
use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::{COLOR, TetrisBlock};

pub struct T {
    pub block_group: BlockGroup,
    pub rotation_position: RotationPosition,
}

impl T {
    pub fn new() -> T {
        T {
            block_group: BlockGroup {
                blocks: [
                    TetrisBlock::new_with_color(5, 19, COLOR::RED),
                    TetrisBlock::new_with_color(4, 18, COLOR::RED),
                    TetrisBlock::new_with_color(5, 18, COLOR::RED),
                    TetrisBlock::new_with_color(6, 18, COLOR::RED),
                ]
            },
            rotation_position: N,
        }
    }
}

impl PlayableShape for T {
    fn move_direction(&self, direction: Direction) -> Box<dyn PlayableShape> {
        let new_position = self.block_group.move_direction(direction);
        Box::new(T { block_group: new_position, rotation_position: self.rotation_position.clone() })
    }

    fn rotate(&self) -> Box<dyn PlayableShape> {
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

        Box::new(T {
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