use crate::direction::Direction;
use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::{COLOR, TetrisBlock};

#[derive(Clone)]
pub struct S {
    pub block_group: BlockGroup,
    pub rotated: bool,
}

impl S {
    pub fn new() -> S {
        S {
            block_group: BlockGroup {
                blocks: [
                    TetrisBlock::new_with_color(4, 19, COLOR::GREEN),
                    TetrisBlock::new_with_color(5, 19, COLOR::GREEN),
                    TetrisBlock::new_with_color(3, 18, COLOR::GREEN),
                    TetrisBlock::new_with_color(4, 18, COLOR::GREEN),
                ]
            },
            rotated: false,
        }
    }
}

impl PlayableShape for S {
    fn move_direction(&self, direction: Direction) -> Box<dyn PlayableShape> {
        let new_position = self.block_group.move_direction(direction);
        Box::new(S { block_group: new_position, rotated: self.rotated })
    }

    fn rotate(&self) -> Box<dyn PlayableShape> {
        let rotated_block = match self.rotated {
            false => {
                [
                    self.block_group.blocks[0].moved(0, -1),
                    self.block_group.blocks[1].moved(-1, -2),
                    self.block_group.blocks[2].moved(0, 1),
                    self.block_group.blocks[3].moved(-1, 0)
                ]
            }
            true => {
                [
                    self.block_group.blocks[0].moved(0, 1),
                    self.block_group.blocks[1].moved(1, 2),
                    self.block_group.blocks[2].moved(0, -1),
                    self.block_group.blocks[3].moved(1, 0)
                ]
            }
        };

        Box::new(S {
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