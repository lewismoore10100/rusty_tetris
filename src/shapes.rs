use crate::tetris_block::TetrisBlock;

pub trait PlayableShape {
    fn all_blocks(&self) -> &[TetrisBlock];
    fn drain_to(&mut self, merge_to: &mut Vec<TetrisBlock>);
    fn move_down(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
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
    fn all_blocks(&self) -> &[TetrisBlock] {
        return &self.blocks[..];
    }

    fn drain_to(&mut self, merge_to: &mut Vec<TetrisBlock>) {
        merge_to.append(&mut self.blocks);
    }

    fn move_down(&mut self) {
        self.blocks.iter_mut().for_each(|b| b.y -= 1)
    }

    fn move_left(&mut self) {
        self.blocks.iter_mut().for_each(|b| b.x -= 1)
    }

    fn move_right(&mut self) {
        self.blocks.iter_mut().for_each(|b| b.x += 1)
    }
}