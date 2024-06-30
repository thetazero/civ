use rocket::serde::{Deserialize, Serialize};

use super::building;

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Tile {
    pub kind: TileKind,
    pub building: Option<building::Building>,
    pub owner: Option<usize>,
}

impl Tile {
    pub fn new(kind: TileKind) -> Self {
        Tile {
            kind,
            building: None,
            owner: None,
        }
    }

    pub fn is_spawnable(self: &Tile) -> bool {
        match self.kind {
            TileKind::Desert => true,
            TileKind::Forest => true,
            TileKind::Mountain => true,
            TileKind::SnowyMountain => true,
            TileKind::Shallows => false,
            TileKind::Ocean => false,
            TileKind::Beach => true,
            TileKind::Unknown => false,
        }
    }
}



#[derive(Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, Debug)]
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
