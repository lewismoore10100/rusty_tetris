use crate::tetris_block::TetrisBlock;

pub struct Square {
    pub blocks: Vec<TetrisBlock>
}

impl Square {
    pub(crate) fn new() -> Square {
        Square{blocks: vec![
            TetrisBlock::new(4, 19),
            TetrisBlock::new(5, 19),
            TetrisBlock::new(4, 18),
            TetrisBlock::new(5, 18)
        ]}
    }

    pub fn hit(&self) -> bool{
        self.blocks[3].y == 0
    }

    pub fn drain_to(&mut self, merge_to: &mut Vec<TetrisBlock>){
        merge_to.append(&mut self.blocks);
    }

    pub fn move_down(&mut self){
        self.blocks.iter_mut().for_each(|b| b.y -= 1)
    }
}