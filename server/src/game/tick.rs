use crate::game::Game;

use super::{empire::Empire, hex::Hex, tile::Tile};

pub fn tick(game: &Game) {
    let mut world_state = game.world_state.lock().unwrap();

    let mut empire_state = game.empire_state.lock().unwrap();

    let (_map, _empire_state) = tick_buildings(&mut world_state.map, &mut empire_state);
}

pub fn tick_buildings<'a, 'b>(
    map: &'a mut Hex<Tile>,
    empires: &'b mut Vec<Empire>,
) -> (&'a mut Hex<Tile>, &'b mut Vec<Empire>) {
    map.for_each(|_, tile| {
        if let Some(owner) = tile.owner {
            if let Some(building) = tile.building {
                // TODO: Add resources to owner
                let empire = empires.get_mut(owner).unwrap();
                let production = building.production();
                empire.inventory.add_items(production);
            }
        }
    });

    (map, empires)
}
