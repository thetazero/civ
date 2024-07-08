use rocket::serde::{Deserialize, Serialize};

use super::{city, resource::Resource, tile::TileKind};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Building {
    pub kind: BuildingKind,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(crate = "rocket::serde")]
pub enum BuildingKind {
    EmpireCapital,
    Capital,
    LumberMill,
    Quarry,
    Farm,
}
pub const ALL_BUILDING_KINDS: [BuildingKind; 5] = [
    BuildingKind::EmpireCapital,
    BuildingKind::Capital,
    BuildingKind::LumberMill,
    BuildingKind::Quarry,
    BuildingKind::Farm,
];

impl Building {
    pub fn production(self: &Self) -> Vec<(Resource, u32)> {
        self.kind.production() // Tile modifiers should be added
    }
}

impl BuildingKind {
    pub fn production(self: &Self) -> Vec<(Resource, u32)> {
        match self {
            Self::EmpireCapital => {
                vec![
                    (Resource::Influence, 1),
                    (Resource::Wood, 3),
                    (Resource::Stone, 3),
                ]
            }
            Self::Capital => {
                vec![]
            }
            Self::LumberMill => {
                vec![(Resource::Wood, 1)]
            }
            Self::Quarry => {
                vec![(Resource::Stone, 1)]
            },
            Self::Farm => {
                vec![(Resource::Food, 1)]
            }
        }
    }

    pub const fn is_city(self: &Self) -> bool {
        matches!(self, Self::Capital | Self::EmpireCapital)
    }

    pub fn can_build_on(self: &Self, tile_kind: &TileKind, city_tile: bool) -> bool {
        match self {
            Self::EmpireCapital => false, // Never buildable
            Self::Capital => false,       // Only buildable by units
            Self::LumberMill => tile_kind.has_trees() && city_tile,
            Self::Quarry => tile_kind.has_stone() && city_tile,
            Self::Farm => tile_kind.is_farmable() && city_tile,
        }
    }

    pub const fn all() -> &'static [Self; 5] {
        &ALL_BUILDING_KINDS
    }
}
