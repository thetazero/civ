use std::collections::HashMap;

use crate::game::{City, Game};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[get("/all")]
pub async fn get_all(game: &State<Game>) -> Json<HashMap<usize, City>> {
    let world_state = &game.world_state.lock().unwrap();

    let cities = world_state.cities.clone();

    Json(cities.cities)
}

#[get("/<id>")]
pub async fn get_by_id(game: &State<Game>, id: usize) -> Result<Json<City>, Status> {
    let world_state = &game.world_state.lock().unwrap();

    match world_state.cities.cities.get(&id) {
        Some(city) => Ok(Json(city.clone())),
        None => Err(Status::NotFound),
    }
}
