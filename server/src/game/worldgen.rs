use crate::game::{WorldState, Tile, TileKind};

pub fn generate() -> WorldState {
    let mut world = WorldState {
        map: vec![]
    };
    for _ in 0..100 {
        let mut row = vec![];
        for _ in 0..100 {
            let tile = Tile {
                kind: TileKind::Desert,
            };
            row.push(tile);
        }
        world.map.push(row);
    }
    world
}
