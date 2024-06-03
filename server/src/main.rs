use game::{hex::HexIndex, Game, tile::Tile};
use rand::Rng;
use rocket::serde::json::Json;
use rocket::State;

mod game;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
async fn random_wait() -> Json<i32> {
    let mut rng = rand::thread_rng();
    let random_int = rng.gen_range(1..=10);

    Json(random_int)
}

#[get("/name")]
async fn game_name(game: &State<Game>) -> Json<String> {
    Json(game.name())
}

#[get("/all")]
async fn game_all(game: &State<Game>) -> Json<Game> {
    let game = game.inner();
    Json(game.clone())
}

#[get("/tile/<row>/<col>")]
async fn get_tile(game: &State<Game>, row: usize, col: usize) -> Json<Tile> {
    let tile = game.world_state.map.get(HexIndex { row, col });
    match tile {
        Some(tile) => Json(*tile),
        None => panic!("Tile not found")
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/random", routes![random_wait])
        .mount("/game", routes![game_name, game_all, get_tile])
        .manage(Game::new())
        .launch()
        .await?;

    Ok(())
}
