use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum Resource {
    Wood,
    Stone,
    Food,
    Influence,
    Production,
}
