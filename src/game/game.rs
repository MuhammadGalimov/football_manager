use std::fs::File;
use std::io::Write;

use super::player::Player;
use super::team::Team;

pub struct Game {
    teams: Vec<Team>,
}

impl Game {
    pub fn new() -> Self {
        let team = Team {
            players: vec![Player { 
                name: "muhagal".to_string(),
                position: super::player::Position::Forward,
                hand: super::player::Hand::Right,
                cost: 100,
            }]
        };

        Game {
            teams: vec![team],
        }
    }

    pub fn save(&self) {
        let mut out = String::from("");
        for team in &self.teams {
            out.push_str(&serde_json::to_string(team).unwrap()[..]);
        }

        let path = "data.txt";
        let mut file = File::create(path).unwrap();
        write!(file, "{}", out).unwrap();
    }
}