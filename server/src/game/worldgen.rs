use crate::game::tile::{Tile, TileData};
use crate::game::WorldState;
use noise::{NoiseFn, Simplex};

use super::hex::{Coordinate, Hex, HexIndex};

const WATER_LEVEL: f32 = 0.2;

fn gen_tile_biome(height: f64, biome: f64) -> Tile {
    if height < -0.2 {
        Tile::Ocean(TileData::default())
    } else if height <= WATER_LEVEL as f64 {
        Tile::Shallows(TileData::default())
    } else if height < 0.5 {
        if biome < 0. {
            Tile::Desert(TileData::default())
        } else {
            Tile::Forest(TileData::default())
        }
    } else if height < 0.7 {
        Tile::Mountain(TileData::default())
    } else {
        Tile::SnowyMountain(TileData::default())
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

pub fn generate() -> WorldState {
    let simplex_2d = Simplex::new(2);

    let mut map = Hex::new(10, 10, Tile::Unknown);
    map.map_mut(|_, index| {
        let coordinate = index.to_coords();
        tile_biome(coordinate, &simplex_2d)
    });

    WorldState { map }
}
