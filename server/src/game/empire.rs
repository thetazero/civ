use super::inventory::Inventory;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Clone)]
pub struct Empire {
    pub inventory: Inventory
}
