use crate::game::Game;

use super::{actions::apply_actions, empire::Empire, hex::Hex, tile::Tile};

pub fn tick(game: &Game) {
    let mut world_state = game.world_state.lock().unwrap();

    let mut empire_state = game.empire_state.lock().unwrap();

    let actions = vec![];

    apply_actions(&mut world_state, &mut empire_state, actions);
    tick_buildings(&mut world_state.map, &mut empire_state);
}

pub fn tick_buildings(map: &mut Hex<Tile>, empires: &mut Vec<Empire>) {
    map.for_each(|_, tile| {
        if let Some(owner) = tile.owner {
            if let Some(building) = tile.building {
                // TODO: Add resources to owner
                let empire = empires.get_mut(owner).unwrap();
                let production = building.production();
                empire.inventory.add_items(production);
            }
        }
    })
}

#[cfg(test)]
mod test {
    use crate::game::{
        building::{Building, BuildingKind},
        hex::HexIndex,
        tile::TileKind,
        Inventory,
    };

    use super::*;

    #[test]
    fn test_tick_buildings() {
        let mut map = Hex::new(2, 2, Default::default());
        let i00 = HexIndex { row: 0, col: 0 };
        map.set(
            i00,
            Tile {
                kind: TileKind::Forest,
                owner: Some(0),
                ..Default::default()
            },
        );

        let empire = Empire {
            inventory: Inventory::empty(),
        };

        let mut empires = vec![empire];

        let building = Building {
            kind: BuildingKind::Farm,
        };

        let i00 = map.get_mut(i00).unwrap();
        i00.building = Some(building);

        tick_buildings(&mut map, &mut empires);

        assert_eq!(empires[0].inventory.food, 1);
    }
}
