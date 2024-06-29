use super::inventory::Inventory;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Clone)]
pub struct Empire {
    pub inventory: Inventory
}

impl Default for Empire {
    fn default() -> Self {
        return Empire{
            inventory: Inventory::default()
        }
    }
}
