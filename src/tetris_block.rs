#[derive(Clone)]
pub struct TetrisBlock {
    pub x: i32,
    pub y: i32,
    pub id: String,
    pub color: COLOR,
}
#[derive(Clone)]
pub enum COLOR {
    BLUE,
    GREEN,
    YELLOW,
    RED,
    ORANGE,
    TURQUOISE
}

impl TetrisBlock {
    pub fn new(x: i32, y: i32) -> TetrisBlock {
        TetrisBlock { x, y, id: format!("{}", generate_id()), color: COLOR::BLUE }
    }

    pub fn new_with_color(x: i32, y: i32, color: COLOR) -> TetrisBlock {
        TetrisBlock { x, y, id: format!("{}", generate_id()), color }
    }

    pub fn moved(&self, x_change: i32, y_change: i32) -> TetrisBlock {
        TetrisBlock { x: self.x + x_change, y: self.y + y_change, id: self.id.clone(), color: self.color.clone() }
    }

    pub fn same_position(&self, other: &TetrisBlock) -> bool {
        self.x == other.x && self.y == other.y
    }
}

static mut NEXT_ID: i32 = 0;

fn generate_id() -> i32 {
    unsafe {
        NEXT_ID += 1;
        NEXT_ID
    }
}





