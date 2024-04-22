use crate::tetris_block::TetrisBlock;

pub struct BlockGroup {
    pub blocks: Vec<TetrisBlock>,
}

impl BlockGroup {
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


pub struct Square {
    pub blocks: BlockGroup
}

impl Square {
    pub(crate) fn new() -> Square {
        Square {
            blocks: BlockGroup {
                blocks: vec![
                    TetrisBlock::new(4, 19),
                    TetrisBlock::new(5, 19),
                    TetrisBlock::new(4, 18),
                    TetrisBlock::new(5, 18),
                ]
            }
        }
    }
}

pub trait PlayableShape {
    fn blocks(&self) -> &[TetrisBlock];

    fn drain_to(&mut self, merge_to: &mut Vec<TetrisBlock>);

    fn move_down(&mut self);

    fn move_left(&mut self);
    fn move_right(&mut self);
}
impl PlayableShape for Square {
    fn blocks(&self) -> &[TetrisBlock] {
        &self.blocks.blocks[..]
    }
    fn drain_to(&mut self, merge_to: &mut Vec<TetrisBlock>){
        self.blocks.drain_to(merge_to)
    }

    fn move_down(&mut self) {
        self.blocks.move_down()
    }

    fn move_left(&mut self) {
        self.blocks.move_left()
    }
    fn move_right(&mut self) {
        self.blocks.move_right()
    }
}