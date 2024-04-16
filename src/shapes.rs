use crate::tetris_block::TetrisBlock;

pub trait PlayableShape {
    fn blocks(&self) -> &[TetrisBlock];
    fn blocks_mut(&mut self) -> &mut Vec<TetrisBlock>;

    fn drain_to(&mut self, merge_to: &mut Vec<TetrisBlock>) {
        merge_to.append(&mut self.blocks_mut());
    }

    fn move_down(&mut self) {
        self.blocks_mut().iter_mut().for_each(|b| b.y -= 1)
    }

    fn move_left(&mut self) {
        self.blocks_mut().iter_mut().for_each(|b| b.x -= 1)
    }

    fn move_right(&mut self) {
        self.blocks_mut().iter_mut().for_each(|b| b.x += 1)
    }
}

pub struct Square {
    pub blocks: Vec<TetrisBlock>,
}

impl Square {
    pub(crate) fn new() -> Square {
        Square {
            blocks: vec![
                TetrisBlock::new(4, 19),
                TetrisBlock::new(5, 19),
                TetrisBlock::new(4, 18),
                TetrisBlock::new(5, 18),
            ]
        }
    }
}
impl PlayableShape for Square {
    fn blocks(&self) -> &[TetrisBlock] {
        &self.blocks[..]
    }
    fn blocks_mut(&mut self) -> &mut Vec<TetrisBlock> {
        &mut self.blocks
    }
}