use crate::rotation_position::RotationPosition::{E, N, S, W};

#[derive(Debug, Clone)]
pub enum RotationPosition {
    N,
    E,
    S,
    W,
}

impl RotationPosition {
    pub fn next_position(&self) -> RotationPosition {
        match self {
            N => { E }
            E => { S }
            S => { W }
            W => { N }
        }
    }
}