use crate::direction::Direction;
use crate::shapes::{BlockGroup, PlayableShape};
use crate::tetris_block::TetrisBlock;

#[derive(Clone)]
pub struct Square {
    pub block_group: BlockGroup,
}

impl Square {
    pub fn new() -> Square {
        Square {
            block_group: BlockGroup {
                blocks: [
                    TetrisBlock::new_with_color(4, 19, [0f32, 0f32, 1f32, 1f32]),
                    TetrisBlock::new_with_color(5, 19, [0f32, 0f32, 1f32, 1f32]),
                    TetrisBlock::new_with_color(4, 18, [0f32, 0f32, 1f32, 1f32]),
                    TetrisBlock::new_with_color(5, 18, [0f32, 0f32, 1f32, 1f32]),
                ]
            }
        }
    }
}

impl PlayableShape for Square {
    fn move_direction(&self, direction: Direction) -> Box<dyn PlayableShape> {
        let new_position = self.block_group.move_direction(direction);
        Box::new(Square { block_group: new_position })
    }

    fn rotate(&self) -> Box<dyn PlayableShape> {
        Box::new(self.clone())
    }

    fn blocks(&self) -> &[TetrisBlock; 4] {
        &self.block_group.blocks
    }
}