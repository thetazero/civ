use crate::game::Game;

use super::{empire::Empire, hex::Hex, tile::Tile};

pub fn tick(game: &mut Game) {
    let (_map, _empire_state) = tick_buildings(&mut game.world_state.map, &mut game.empire_state);
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
