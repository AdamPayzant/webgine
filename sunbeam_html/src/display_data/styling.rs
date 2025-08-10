use crate::types;
use crate::Document;

#[derive(Debug, Default)]
pub enum Layouts {
    Flex,
    Grid,
    #[default]
    None,
}

pub struct Styling {
    pub position: types::Positioning,
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
    pub layout: Layouts,
}

impl Default for Styling {
    fn default() -> Self {
        Self {
            position: types::Positioning::default(),
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            layout: Layouts::default(),
        }
    }
}

impl Styling {
    pub fn from_strings(doc: &Document, class: &str, strings: &Vec<String>) -> Self {
        let mut res = Self::default();
        // Set class styling first, then inline styling
        res
    }
}
