use crate::game::tile::{Tile, TileKind};
use crate::game::WorldState;
use noise::{NoiseFn, Simplex};
use rand::Rng;
use std::collections::HashSet;

use super::building;
use super::hex::{Coordinate, Hex, HexIndex};
use super::Game;
use super::empire::Empire;

const WATER_LEVEL: f32 = 0.2;

fn gen_tile_biome(height: f64, biome: f64) -> Tile {
    if height < -0.2 {
        Tile::new(TileKind::Ocean)
    } else if height <= WATER_LEVEL as f64 {
        Tile::new(TileKind::Shallows)
    } else if height < 0.5 {
        if biome < 0. {
            Tile::new(TileKind::Desert)
        } else {
            Tile::new(TileKind::Forest)
        }
    } else if height < 0.7 {
        Tile::new(TileKind::Mountain)
    } else {
        Tile::new(TileKind::SnowyMountain)
    }
}

fn scaled_simplex_2d(simplex: Simplex, x: f64, y: f64, scale: f64) -> f64 {
    simplex.get([x * scale, y * scale])
}

pub fn tile_biome(coordinate: Coordinate, simplex_2d: &Simplex) -> Tile {
    let x_float = coordinate.x;
    let y_float = coordinate.y;

    let height = scaled_simplex_2d(*simplex_2d, x_float, y_float, 0.05 / 2.);
    let biome = scaled_simplex_2d(*simplex_2d, x_float, y_float, 0.02 / 2.);

    gen_tile_biome(height, biome)
}

pub fn pick_empire_locations(
    map: &Hex<Tile>,
    empire_count: usize,
    rows: usize,
    cols: usize,
) -> HashSet<HexIndex> {
    let mut empire_hexes = HashSet::new();
    let mut rng = rand::thread_rng();

    let mut empires_created = 0;
    let max_atempts = 1000;
    for _ in 0..max_atempts {
        let row = rng.gen_range(0..rows);
        let col = rng.gen_range(0..cols);
        let index = HexIndex { row, col };
        if empire_hexes.contains(&index) {
            continue;
        }
        if !map.get(index).unwrap().is_spawnable() {
            continue;
        }
        empire_hexes.insert(index);
        empires_created += 1;
        if empires_created == empire_count {
            return empire_hexes;
        }
    }
    panic!(
        "Could not find enough unique locations for empires, (map size too small, or rng wrong)"
    );
}

fn place_empires(map: &mut Hex<Tile>, empire_locations: HashSet<HexIndex>) -> &mut Hex<Tile> {
    let mut empire_id = 0;
    for index in empire_locations {
        let tile = map.get_mut(index).unwrap();
        tile.building = Some(building::Building {
            kind: building::BuildingKind::Capital,
        });
        tile.owner = Some(empire_id);
        empire_id += 1;
    }

    map
}

pub fn generate(config: WorldGenConfig) -> Game {
    let simplex_2d = Simplex::new(2);

    let mut map = Hex::new(config.rows, config.cols, Default::default());
    map.map(|_, index| {
        let coordinate = index.to_coords();
        tile_biome(coordinate, &simplex_2d)
    });

    let empire_locations =
        pick_empire_locations(&map, config.empire_count, config.rows, config.cols);
    place_empires(&mut map, empire_locations);

    let mut empires = vec![];

    for _ in 0..config.empire_count {
        empires.push(
            Empire::default()
        )
    }

    Game {
        world_state: WorldState { map },
        empire_state: empires
    }
}

pub struct WorldGenConfig {
    pub rows: usize,
    pub cols: usize,
    pub empire_count: usize,
}

impl Default for WorldGenConfig {
    fn default() -> Self {
        return WorldGenConfig{
            rows: 10,
            cols: 10,
            empire_count: 3
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empire_count(){
        let mut config = WorldGenConfig::default();
        config.empire_count = 3;

        let game = generate(config);

        assert_eq!(game.empire_state.len(), 3);
    }

}
