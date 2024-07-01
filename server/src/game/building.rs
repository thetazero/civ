use rocket::serde::{Deserialize, Serialize};

use super::resource::Resource;

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Building {
    pub kind: BuildingKind,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(crate = "rocket::serde")]
pub enum BuildingKind {
    Capital,
    City,
}

impl Building {
    pub fn production(self: &Building) -> Vec<(Resource, u32)> {
        match self.kind {
            BuildingKind::Capital => {
                vec![
                    (Resource::Influence, 1),
                    (Resource::Wood, 3),
                    (Resource::Stone, 3),
                ]
            }
            BuildingKind::City => {
                vec![]
            }
        }
    }
}
