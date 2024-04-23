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

pub trait PlayableShape {
    fn blocks(&self) -> &[TetrisBlock] {
        &self.block_group().blocks[..]
    }
    fn drain_to(&mut self, merge_to: &mut Vec<TetrisBlock>) {
        self.block_group_mut().drain_to(merge_to)
    }
    fn move_down(&mut self) {
        self.block_group_mut().move_down()
    }
    fn move_left(&mut self) {
        self.block_group_mut().move_left()
    }
    fn move_right(&mut self) {
        self.block_group_mut().move_right()
    }
    fn block_group_mut(&mut self) -> &mut BlockGroup;
    fn block_group(&self) -> &BlockGroup;
    fn rotate(&mut self){}
}
