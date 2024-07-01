use std::collections::HashMap;

use rocket::serde::{Deserialize, Serialize};

use super::{
    building::{Building, BuildingKind},
    hex::{Hex, HexIndex},
    tile::Tile,
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CitiesState {
    pub cities: HashMap<usize, City>,
    next_id: usize,
}

impl CitiesState {
    pub fn next_id(&mut self) -> usize {
        let cur = self.next_id;
        self.next_id += 1;
        cur
    }
}

impl Default for CitiesState {
    fn default() -> Self {
        CitiesState {
            cities: HashMap::new(),
            next_id: 0,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct City {
    pub tiles: Vec<HexIndex>,
    pub owner: usize,
}

fn unclaimed_neighbor_tiles(index: &HexIndex, map: &Hex<Tile>) -> Vec<HexIndex> {
    let tiles = index.neighbors();
    tiles
        .iter()
        .filter(|index| match map.get(**index) {
            Some(tile) => tile.city == None,
            None => false,
        })
        .copied()
        .collect()
}

impl City {
    pub fn new_capital(
        owner_id: usize,
        city_id: usize,
        home_index: &HexIndex,
        map: &mut Hex<Tile>,
    ) -> City {
        let map = map;

        let mut tiles = unclaimed_neighbor_tiles(home_index, map);
        tiles.push(*home_index);

        tiles.iter().for_each(|index| match map.get_mut(*index) {
            Some(tile) => {
                tile.city = Some(city_id);
                tile.owner = Some(owner_id);
            }
            None => {}
        });

        let home_tile = map.get_mut(*home_index).unwrap();
        home_tile.building = Some(Building {
            kind: BuildingKind::Capital,
        });

        City {
            tiles: tiles,
            owner: owner_id,
        }
    }

    pub fn new_city(
        owner_id: usize,
        city_id: usize,
        index: &HexIndex,
        map: &mut Hex<Tile>,
    ) -> City {
        let tile = map.get_mut(*index).unwrap();
        assert_eq!(tile.city, None);

        tile.city = Some(city_id);
        tile.owner = Some(owner_id);
        tile.building = Some(Building {
            kind: BuildingKind::City,
        });

        City {
            tiles: vec![*index],
            owner: owner_id,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::game::{
        hex::HexIndex,
        tile::TileKind,
        worldgen::{self, WorldGenConfig},
    };

    use super::super::hex::HexDirection;
    use super::*;

    #[test]
    fn capitals_dont_overlap() {
        let cfg = WorldGenConfig::tiny();

        let mut map = worldgen::generate_map(&cfg);

        let cap1id = 1;
        let cap2id = 2;

        let hex_index1 = HexIndex { row: 3, col: 3 };
        let hex_index2 = hex_index1
            .offset(&HexDirection::DownLeft)
            .offset(&HexDirection::DownLeft);

        let cap1 = City::new_capital(0, cap1id, &hex_index1, &mut map);
        let cap2 = City::new_capital(0, cap2id, &hex_index2, &mut map);

        hex_index1.neighbors().iter().for_each(|idx| {
            let tile = map.get(*idx).unwrap();
            assert_eq!(tile.city, Some(cap1id));
        });
        assert_eq!(cap1.tiles.len(), 7);

        assert_eq!(
            hex_index2
                .neighbors()
                .iter()
                .filter(|idx| { map.get(**idx).unwrap().city == Some(cap2id) })
                .count(),
            5
        );
        assert_eq!(cap2.tiles.len(), 6);

        let cap1_building = map.get(hex_index1).unwrap().building;
        assert_ne!(cap1_building, None);
        assert_eq!(cap1_building.unwrap().kind, BuildingKind::Capital);
    }
}
