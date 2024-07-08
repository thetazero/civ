use super::{building::BuildingKind, hex::HexIndex, tile::Tile, City, Empire, Hex, WorldState};

use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(crate = "rocket::serde")]
pub enum Action {
    Build(BuildAction),
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BuildAction {
    pub building: BuildingKind,
    pub location: HexIndex,
}

pub fn make_movelists(world_state: WorldState, empire_state: Vec<Empire>) -> Vec<Vec<Action>> {
    let empire_count = empire_state.len();
    let mut movelists = Vec::with_capacity(empire_count);
    for _ in 0..empire_count {
        movelists.push(Vec::new());
    }

    let cities = world_state.cities.cities;

    for (city_id, city) in cities {
        let empire_id = city.owner;
        let empire = &empire_state[empire_id];

        let map = &world_state.map;

        let city_movelists = city_moves(map, &city, empire);

        movelists[empire_id].extend(city_movelists);
    }

    movelists
}

fn city_moves(map: &Hex<Tile>, city: &City, _empire: &Empire) -> Vec<Action> {
    let mut movelist = Vec::new();

    for tile_index in &city.tiles {
        let tile = map.get(*tile_index).unwrap();

        match tile.building {
            Some(building) => {}
            None => {
                for building_kind in BuildingKind::all() {
                    let can_build = building_kind.can_build_on(&tile.kind, true);
                    // TODO: Can afford check
                    if can_build {
                        movelist.push(Action::Build(BuildAction {
                            building: *building_kind,
                            location: *tile_index,
                        }));
                    }
                }
            }
        }
    }

    movelist
}

#[cfg(test)]
mod test {
    use crate::game::{hex::HexIndex, tile::TileKind, Inventory};

    use super::*;

    #[test]
    fn test_city_moves() {
        let mut map = Hex::new(2, 2, Default::default());
        let i00 = HexIndex { row: 0, col: 0 };
        let i01 = HexIndex { row: 0, col: 1 };
        let i10 = HexIndex { row: 1, col: 0 };
        map.set(
            i00,
            Tile {
                kind: TileKind::Forest,
                ..Default::default()
            },
        );

        let city = City {
            tiles: vec![i00, i01],
            owner: 0,
        };
        let empire = Empire {
            inventory: Inventory::empty(),
        };
        let moves = city_moves(&map, &city, &empire);
        assert_eq!(
            moves,
            vec![
                Action::Build(BuildAction {
                    building: BuildingKind::LumberMill,
                    location: i00,
                }),
                Action::Build(BuildAction {
                    building: BuildingKind::Factory,
                    location: i00,
                }),
            ]
        );

        map.set(
            i01,
            Tile {
                kind: TileKind::Mountain,
                ..Default::default()
            },
        );
        map.set(
            i00,
            Tile {
                kind: TileKind::Unknown,
                ..Default::default()
            },
        );
        let moves = city_moves(&map, &city, &empire);
        assert_eq!(
            moves,
            vec![
                Action::Build(BuildAction {
                    building: BuildingKind::Quarry,
                    location: i01,
                }),
                Action::Build(BuildAction {
                    building: BuildingKind::Factory,
                    location: i01,
                }),
            ]
        );

        map.set(
            i10,
            Tile {
                kind: TileKind::Mountain,
                ..Default::default()
            },
        );
        let moves = city_moves(&map, &city, &empire);
        assert_eq!(
            moves,
            vec![
                Action::Build(BuildAction {
                    building: BuildingKind::Quarry,
                    location: i01,
                }),
                Action::Build(BuildAction {
                    building: BuildingKind::Factory,
                    location: i01,
                }),
            ]
        );
    }
}
