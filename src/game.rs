use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub position: Position,
    pub hand: Hand,
    pub cost: u32
}

#[derive(Serialize, Deserialize)]
pub enum Position {
    Goalkeeper,
    Defender,
    Halfback,
    Forward,
}

#[derive(Serialize, Deserialize)]
pub enum Hand {
    Left,
    Right
}