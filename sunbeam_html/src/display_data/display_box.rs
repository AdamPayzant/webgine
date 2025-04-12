use super::border::Border;
use super::img::Img;
use super::text::Text;

pub enum DisplayBoxData {
    Text(Text),
    Img(Img),
    None,
}

#[derive(Default)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
}

// The top left position of the element
pub enum Coordinates {
    // Relative to the parent box
    Relative(Coord),
    // Absolute values on the screen
    Absolute(Coord),
}

pub struct DisplayBox {
    pub id: usize,
    pub data: DisplayBoxData,
    pub children: Vec<DisplayBox>,
    pub coords: Coordinates,
    pub height: f32,
    pub width: f32,
    pub background: Option<Vec<u8>>,
    pub border: Option<Border>,
}

impl DisplayBox {
    pub fn new() -> DisplayBox {
        DisplayBox {
            id: 0,
            data: DisplayBoxData::None,
            children: Vec::new(),
            coords: Coordinates::Relative(Coord::default()),
            height: 0.0,
            width: 0.0,
            background: None,
            border: None,
        }
    }
}
