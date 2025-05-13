use super::border::Border;
use super::img::Img;
use super::styling::Styling;
use super::text::Text;
use crate::types;

pub enum DisplayBoxData {
    Text(Text),
    Img(Img),
    None,
}

pub struct DisplayBox {
    pub id: usize,
    pub data: DisplayBoxData,
    pub children: Vec<DisplayBox>,
    pub style: Styling,
    pub background: Option<Vec<u8>>,
    pub border: Option<Border>,
}

impl DisplayBox {
    pub fn new() -> DisplayBox {
        DisplayBox {
            id: 0,
            data: DisplayBoxData::None,
            children: Vec::new(),
            style: Styling::default(),
            background: None,
            border: None,
        }
    }
}
