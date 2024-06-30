use game::Game;
use rocket::serde::json::Json;
use rocket::State;

mod empire_endpoint;
mod game;
mod tile_endpoint;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
async fn tick(game: &State<Game>) -> Json<bool> {
    match game.ready() {
        true => {
            game.tick();
            Json(true)
        }
        false => Json(false),
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount(
            "/tile",
            routes![tile_endpoint::get_tile, tile_endpoint::all_tiles],
        )
        .mount("/tick", routes![tick])
        .mount(
            "/empire",
            routes![empire_endpoint::get_inventory, empire_endpoint::count],
        )
        .manage(Game::new())
        .launch()
        .await?;

    Ok(())
}
