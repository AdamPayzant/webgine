use crate::types;

pub struct Styling {
    pub position: types::Positioning,
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
}

impl Default for Styling {
    fn default() -> Self {
        Self {
            position: types::Positioning::default(),
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
        }
    }
}
