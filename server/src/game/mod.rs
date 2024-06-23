use crate::game::hex::Hex;
use rocket::serde::{Deserialize, Serialize};

pub mod hex;
pub mod tile;

mod building;
mod resource;
mod worldgen;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Clone)]
pub struct WorldState {
    pub map: Hex<tile::Tile>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Clone)]
pub struct Game {
    pub world_state: WorldState,
}

impl Game {
    pub fn new() -> Self {
        Game {
            world_state: worldgen::generate(),
        }
    }

    pub fn name(self: &Game) -> String {
        "Game Name".to_string()
    }
}
