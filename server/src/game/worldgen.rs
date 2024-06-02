use crate::game::tile::{Tile, TileData};
use crate::game::WorldState;
use noise::{NoiseFn, Simplex};

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

pub fn spawn_empty_map() -> Vec<Vec<Tile>> {
    let mut map = vec![];
    for _ in 0..10 {
        let mut row = vec![];
        for _ in 0..10 {
            let tile = Tile::Unknown;
            row.push(tile);
        }
        map.push(row);
    }
    map
}

pub fn spawn_biomes(mut map: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let simplex_2d = Simplex::new(2);

    for (x, row) in map.iter_mut().enumerate() {
        for (y, tile) in row.iter_mut().enumerate() {
            let x_float = x as f64;
            let y_float = y as f64;

            let height = scaled_simplex_2d(simplex_2d, x_float, y_float, 0.05 / 2.);
            let biome = scaled_simplex_2d(simplex_2d, x_float, y_float, 0.02 / 2.);

            *tile = gen_tile_biome(height, biome)
        }
    }

    map
}

pub fn generate() -> WorldState {
    let mut map = spawn_empty_map();
    map = spawn_biomes(map);

    WorldState { map }
}
