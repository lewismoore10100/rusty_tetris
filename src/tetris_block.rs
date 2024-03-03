pub struct TetrisBlock {
    pub x: u32,
    pub y: u32
}

impl TetrisBlock {
    pub fn name(&self) -> String {
        format!("{}{}", self.x, self.y)
    }
}



