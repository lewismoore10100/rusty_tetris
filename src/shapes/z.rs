use crate::direction::Direction;
use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::{COLOR, TetrisBlock};

#[derive(Clone)]
pub struct Z {
    pub block_group: BlockGroup,
    pub rotated: bool
}

impl Z {
    pub fn new() -> Z {
        Z {
            block_group: BlockGroup {
                blocks: [
                    TetrisBlock::new_with_color(3, 19, COLOR::YELLOW),
                    TetrisBlock::new_with_color(4, 19, COLOR::YELLOW),
                    TetrisBlock::new_with_color(4, 18, COLOR::YELLOW),
                    TetrisBlock::new_with_color(5, 18, COLOR::YELLOW),
                ]
            },
            rotated: false,
        }
    }
}

impl PlayableShape for Z {
    fn move_direction(&self, direction: Direction) -> Box<dyn PlayableShape> {
        let new_position = self.block_group.move_direction(direction);
        Box::new(Z { block_group: new_position, rotated: self.rotated })
    }

    fn rotate(&self) -> Box<dyn PlayableShape> {
        let rotated_block = match self.rotated {
            false => {
                [
                    self.block_group.blocks[0].moved(1, 0),
                    self.block_group.blocks[1].moved(0, -1),
                    self.block_group.blocks[2].moved(-1, 0),
                    self.block_group.blocks[3].moved(-2, -1)
                ]
            }
            true => {
                [
                    self.block_group.blocks[0].moved(-1, 0),
                    self.block_group.blocks[1].moved(0, 1),
                    self.block_group.blocks[2].moved(1, 0),
                    self.block_group.blocks[3].moved(2, 1)
                ]
            }
        };

        Box::new(Z {
            block_group: BlockGroup {
                blocks: rotated_block
            },
            rotated: !self.rotated
        })
    }

    fn blocks(&self) -> &[TetrisBlock; 4] {
        &self.block_group.blocks
    }
}