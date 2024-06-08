use crate::game::tile::{Tile, TileKind};
use crate::game::WorldState;
use noise::{NoiseFn, Simplex};
use rand::Rng;
use std::collections::HashSet;

use super::hex::{Coordinate, Hex, HexIndex};

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

pub fn pick_empire_locations(empire_count: usize, rows: usize, cols: usize) -> HashSet<HexIndex> {
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

pub fn generate() -> WorldState {
    let rows = 50;
    let cols = 50;
    let empire_count = 3;

    let simplex_2d = Simplex::new(2);

    let mut map = Hex::new(rows, cols, Default::default());
    map.map(|_, index| {
        let coordinate = index.to_coords();
        tile_biome(coordinate, &simplex_2d)
    });
    let empire_locations = pick_empire_locations(empire_count, rows, cols);

    WorldState { map }
}
