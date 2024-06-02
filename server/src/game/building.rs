pub enum Building {
    Capital(Capital),
}

pub struct Capital {}

impl Building {
    pub fn name(self: &Building) -> String {
        match self {
            Building::Capital(_) => "Capital".to_string(),
        }
    }
}
