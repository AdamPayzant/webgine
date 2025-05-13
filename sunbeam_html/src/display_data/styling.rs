use crate::types;

pub struct Styling {
    pub position: types::Positioning,
}

impl Default for Styling {
    fn default() -> Self {
        Self {
            position: types::Positioning::default(),
        }
    }
}
