use serde::{Serialize, Deserialize};
use super::player::Player;

#[derive(Serialize, Deserialize)]
pub struct Team {
    pub players: Vec<Player>
}

impl Team {

}