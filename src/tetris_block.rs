use rand::Rng;
use serde::Serialize;

#[derive(Eq, Hash, PartialEq, Clone, Serialize)]
pub struct TetrisBlock {
    pub x: u32,
    pub y: u32,
    pub id: String,
}

impl TetrisBlock {
    pub fn new(x: u32, y: u32) -> TetrisBlock {
        TetrisBlock { x, y, id: random_string() }
    }

    pub fn same_position(&self, other: &TetrisBlock) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn random_string() -> String {
    (0..100).map(|_| char::from(rand::thread_rng().gen_range(32..127))).collect()
}





