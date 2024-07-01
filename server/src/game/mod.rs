use rocket::serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use empire::Empire;
use worldgen::WorldGenConfig;

pub mod hex;
pub mod tile;

mod actions;
mod building;
mod city;
mod empire;
mod inventory;
mod movelist;
mod resource;
mod tick;
mod worldgen;

pub use city::{CitiesState, City};
pub use hex::Hex;
pub use inventory::Inventory;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Clone)]
pub struct WorldState {
    pub map: Hex<tile::Tile>,
    pub cities: CitiesState,
}

pub struct Game {
    pub world_state: Arc<Mutex<WorldState>>,
    pub empire_state: Arc<Mutex<Vec<Empire>>>,
}

impl Game {
    pub fn new() -> Self {
        worldgen::generate(WorldGenConfig {
            rows: 30,
            cols: 30,
            empire_count: 10,
        })
    }

    pub fn tick(self: &Game) {
        tick::tick(self);
    }

    pub fn ready(self: &Game) -> bool {
        true
    }
}
