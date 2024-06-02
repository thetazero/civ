use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub enum Tile {
    Desert(TileData),
    Forest(TileData),
    Mountain(TileData),
    SnowyMountain(TileData),
    Shallows(TileData),
    Ocean(TileData),
    Beach(TileData),
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct TileData {}
