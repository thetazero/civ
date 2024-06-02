use rocket::serde::{Deserialize, Serialize};
use crate::game::hex::Hex;

mod building;
mod worldgen;
mod tile;
mod hex;

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
