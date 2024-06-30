use crate::game::{hex::HexIndex, tile::Tile, Game};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

#[get("/row/<row>/col/<col>")]
pub async fn get_tile(game: &State<Game>, row: usize, col: usize) -> Json<Tile> {
    let world_state = &game.world_state.lock().unwrap();

    let tile = world_state.map.get(HexIndex { row, col });
    match tile {
        Some(tile) => Json(*tile),
        None => panic!("Tile not found"),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct IndexedHex {
    idx: HexIndex,
    tile: Tile,
}

#[get("/all")]
pub async fn all_tiles(game: &State<Game>) -> Json<Vec<IndexedHex>> {
    let world_state = &game.world_state.lock().unwrap();

    Json(world_state.map.collect(|index, tile| IndexedHex {
        idx: *index,
        tile: *tile,
    }))
}
