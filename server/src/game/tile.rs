use rocket::serde::{Deserialize, Serialize};

use super::building;

#[derive(Clone, Copy, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Tile {
    pub kind: TileKind,
    pub building: Option<building::Building>,
    pub owner: Option<usize>,
    pub city: Option<usize>,
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
    Plains,
    #[default]
    Unknown,
}

impl Tile {
    pub fn new(kind: TileKind) -> Self {
        Tile {
            kind,
            building: None,
            owner: None,
            city: None,
        }
    }

    pub const fn is_spawnable(self: &Tile) -> bool {
        self.kind.is_spawnable()
    }
}

impl TileKind {
    pub const fn is_spawnable(self: &Self) -> bool {
        match self {
            TileKind::Desert => true,
            TileKind::Forest => true,
            TileKind::Mountain => true,
            TileKind::SnowyMountain => true,
            TileKind::Shallows => false,
            TileKind::Ocean => false,
            TileKind::Beach => true,
            TileKind::Plains => true,
            TileKind::Unknown => false,
        }
    }

    pub const fn has_trees(self: &Self) -> bool {
        matches!(self, TileKind::Forest)
    }

    pub const fn has_stone(self: &Self) -> bool {
        matches!(self, TileKind::Mountain | TileKind::SnowyMountain)
    }

    pub const fn is_farmable(self: &Self) -> bool {
        matches!(self, TileKind::Plains)
    }
}
