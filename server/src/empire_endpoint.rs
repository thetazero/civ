use crate::game;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[get("/<empire_id>/inventory")]
pub async fn get_inventory(
    game: &State<game::Game>,
    empire_id: usize,
) -> Result<Json<game::Inventory>, Status> {
    let empire_state = &game.empire_state.lock().unwrap();

    let empire = empire_state.get(empire_id);

    match empire {
        Some(empire) => Result::Ok(Json(empire.inventory)),
        None => Result::Err(Status::NotFound)
    }
}

#[get("/count")]
pub async fn count(
    game: &State<game::Game>
) -> Json<usize> {
    return Json(game.empire_state.lock().unwrap().len());
}
