use super::resource::Resource;
use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Inventory {
    wood: u32,
    stone: u32,
    influence: u32,
    food: u32,
}

impl Inventory {
    pub fn add_item(self: &mut Self, resource: Resource, amount: u32) -> &mut Self {
        match resource {
            Resource::Influence => self.influence += amount,
            Resource::Wood => self.wood += amount,
            Resource::Stone => self.stone += amount,
            Resource::Food => self.food += amount,
        }
        self
    }

    pub fn add_items(self: &mut Self, resources: Vec<(Resource, u32)>) -> &mut Self{
        for (resource, amount) in resources {
            self.add_item(resource, amount);
        }
        self
    }

    pub fn empty() -> Inventory {
        Inventory {
            wood: 0,
            stone: 0,
            influence: 0,
            food: 0,
        }
    }
}

impl Default for Inventory {
    fn default() -> Inventory {
        Inventory::empty()
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn add_item() {
        let mut empty_inv = Inventory::empty();

        empty_inv.add_item(Resource::Influence, 3);

        assert_eq!(empty_inv.influence, 3);
    }

    #[test]
    fn add_items() {
        let mut empty_inv = Inventory::empty();

        let produce = vec![
            (Resource::Wood, 3),
            (Resource::Stone, 2),
            (Resource::Wood, 1)
        ];

        empty_inv.add_items(produce);

        assert_eq!(empty_inv.influence, 0);
        assert_eq!(empty_inv.wood, 4);
        assert_eq!(empty_inv.stone, 2);
    }
}
