use serde::{Serialize, Deserialize};
use serde_json::Result;
use crate::engine::{
    Button,
    ToButton, 
    ButtonBuilder
};
use crate::env::Env;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub position: Position,
    pub hand: Hand,
    pub cost: u32
}

impl ToButton for Player {
    fn to_button(&self, jump: fn(&mut Env) -> Option<usize>) -> Button {
        ButtonBuilder::new(&(self.name.clone())[..]).jump(jump).build()
    }
}

impl Player {
    fn new(name: String, position: Position, hand: Hand, cost: u32) -> Self {
        Player { name, position, hand, cost }
    }

    fn from_str(str: &str) -> Result<Self> {
        let obj: Player = serde_json::from_str(str)?;
        Ok(obj)
    }

    fn to_str(&self) -> Result<String> {
        let obj = Player {
            name: self.name.clone(),
            position: self.position,
            hand: self.hand,
            cost: self.cost
        };

        let json = serde_json::to_string(&obj)?;
        Ok(json)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Position {
    Goalkeeper,
    Defender,
    Halfback,
    Forward,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Hand {
    Left,
    Right
}