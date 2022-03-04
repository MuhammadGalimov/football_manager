use serde::{Serialize, Deserialize};

use std::fs::File;
use std::io::{Write, self, BufRead};

pub struct Env {
    teams: Vec<Team>,
}

impl Env {
    pub fn new() -> Self {
        let team = Team {
            players: vec![Player { 
                name: "muhagal".to_string(),
                position: Position::Forward,
                hand: Hand::Right,
                cost: 100,
            }]
        };

        Env {
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

    pub fn load_new_game(path: &str) {
        let file = File::open(path).unwrap();
        let lines = io::BufReader::new(file).lines();

        for line in lines {
            if let Ok(item) = line {
                
            }
        }
    }

    pub fn load_game(game_name: &str) -> Self {
        unimplemented!();
    }
}

struct MyTeam {
    team: Team,
    budget: u64,
    env: Env,
    schedule: Vec<Event>
}

impl MyTeam {
    fn new(team: Team, env: Env) -> Self {
        MyTeam { 
            team, 
            budget: 100, 
            env,
            schedule: vec![] 
        }
    }


}

enum Event {
    Training,
    Game(Team)
}

#[derive(Serialize, Deserialize)]
pub struct Team {
    pub players: Vec<Player>
}

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