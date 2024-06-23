use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Building {
    pub kind: BuildingKind,
    pub owner: i32,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum BuildingKind {
    Capital,
}

impl Building {
    pub fn name(self: &Building) -> String {
        return self.kind.name();
    }
}

impl BuildingKind {
    pub fn name(self: &BuildingKind) -> String {
        match self {
            BuildingKind::Capital => "Capital".to_string(),
        }
    }
}
