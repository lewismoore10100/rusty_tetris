use crate::direction::Direction;
use crate::rotation_position::RotationPosition;
use crate::rotation_position::RotationPosition::N;
use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::TetrisBlock;

#[derive(Clone)]
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
    fn move_direction(&self, direction: Direction) -> Box<dyn PlayableShape> {
        let new_position = self.block_group.move_direction(direction);
        Box::new(S { block_group: new_position, rotation_position: self.rotation_position.clone() })
    }

    fn rotate(&self) -> Box<dyn PlayableShape> {
        let rotated_block = match self.rotation_position {
            N => {
                [
                    self.block_group.blocks[0].moved(0, -1),
                    self.block_group.blocks[1].moved(-1, -2),
                    self.block_group.blocks[2].moved(0, 1),
                    self.block_group.blocks[3].moved(-1, 0)
                ]
            }
            _ => { self.block_group.blocks.clone() }
        };

        Box::new(S {
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