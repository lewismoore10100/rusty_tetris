use std::fmt::format;

pub struct TetrisBlock {
    pub x: i32,
    pub y: i32,
    pub id: String,
    pub color: [f32; 4]
}

impl TetrisBlock {
    pub fn new(x: i32, y: i32) -> TetrisBlock {
        TetrisBlock { x, y, id: format!("{}", generate_id()), color: [0f32, 0f32, 1f32, 1f32] }
    }

    pub fn new_with_color(x: i32, y: i32, color: [f32; 4]) -> TetrisBlock {
        TetrisBlock { x, y, id: format!("{}", generate_id()), color}
    }

    pub fn from_with_new_position(existing: &TetrisBlock, x: i32, y: i32) -> TetrisBlock {
        TetrisBlock { x, y, id: existing.id.clone(), color: existing.color}
    }

    pub fn same_position(&self, other: &TetrisBlock) -> bool {
        self.x == other.x && self.y == other.y
    }
}

static mut next_id: i32 = 0;
fn generate_id() -> i32 {
    unsafe {
        next_id += 1;
        next_id
    }
}





