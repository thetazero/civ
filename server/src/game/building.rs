use rocket::{
    serde::{Deserialize, Serialize},
    Build,
};

use super::resource::Resource;

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Building {
    pub kind: BuildingKind,
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

    pub fn production(self: &Building) -> Vec<(Resource, u32)> {
        match self.kind {
            BuildingKind::Capital => {
                vec![
                    (Resource::Influence, 1),
                    (Resource::Wood, 3),
                    (Resource::Stone, 3),
                ]
            }
        }
    }
}

impl BuildingKind {
    pub fn name(self: &BuildingKind) -> String {
        match self {
            BuildingKind::Capital => "Capital".to_string(),
        }
    }
}
