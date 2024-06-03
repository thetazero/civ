use rocket::serde::{Deserialize, Serialize};

use super::building;

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Tile {
    pub kind: TileKind,
    pub building: Option<building::Building>,
}

impl Tile {
    pub fn new(kind: TileKind) -> Self {
        Tile {
            kind,
            building: None,
        }
    }
}



#[derive(Clone, Copy, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub enum TileKind {
    Desert,
    Forest,
    Mountain,
    SnowyMountain,
    Shallows,
    Ocean,
    Beach,
    #[default]
    Unknown,
}
