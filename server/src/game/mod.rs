use crate::game::hex::Hex;
use empire::Empire;
use rocket::serde::{Deserialize, Serialize};
use worldgen::WorldGenConfig;

pub mod hex;
pub mod tile;

mod building;
mod empire;
mod inventory;
mod resource;
mod tick;
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
    pub empire_state: Vec<Empire>,
}

impl Game {
    pub fn new() -> Self {
        worldgen::generate(WorldGenConfig {
            rows: 30,
            cols: 30,
            empire_count: 10,
        })
    }

    pub fn name(self: &Game) -> String {
        "Game Name".to_string()
    }

    pub fn tick(self: &mut Game) {
        tick::tick(self);
    }
}
