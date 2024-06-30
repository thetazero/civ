use game::{hex::HexIndex, tile::Tile, Game};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

mod game;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/name")]
async fn game_name(game: &State<Game>) -> Json<String> {
    Json(game.name())
}

#[get("/tile/<row>/<col>")]
async fn get_tile(game: &State<Game>, row: usize, col: usize) -> Json<Tile> {
    let world_state = &game.world_state.lock().unwrap();

    let tile = world_state.map.get(HexIndex { row, col });
    match tile {
        Some(tile) => Json(*tile),
        None => panic!("Tile not found"),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct IndexedHex {
    idx: HexIndex,
    tile: Tile,
}

#[get("/tile/all")]
async fn all_tiles(game: &State<Game>) -> Json<Vec<IndexedHex>> {
    let world_state = &game.world_state.lock().unwrap();

    Json(world_state.map.collect(|index, tile| IndexedHex {
        idx: *index,
        tile: *tile,
    }))
}

#[get("/")]
async fn tick(game: &State<Game>) -> Json<bool> {
    if game.ready() {
        // TODO: Launch tick in background
        game.tick();
        return Json(true);
    }
    return Json(false);
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/game", routes![game_name, get_tile, all_tiles])
        .mount("/tick", routes![tick])
        .manage(Game::new())
        .launch()
        .await?;

    Ok(())
}
