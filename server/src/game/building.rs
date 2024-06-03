use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Building {
    Capital(Capital),
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Capital {}

impl Building {
    pub fn name(self: &Building) -> String {
        match self {
            Building::Capital(_) => "Capital".to_string(),
        }
    }
}
