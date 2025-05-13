use crate::display_data::display_box::{self, DisplayBox};
use crate::document::doctree;

pub enum QuirksMode {
    Off,
    LimitedQuirks,
    Quirks,
}

pub struct Document {
    pub doctree: doctree::Doctree,
    doctype: String,
    quirksmode: QuirksMode,
    pending_parsing_blocking_script: Option<()>, // TODO

    width: u64,
    height: u64,
}

impl Document {
    pub fn new() -> Document {
        Document {
            doctree: doctree::Doctree::new(),
            doctype: "".to_string(),
            quirksmode: QuirksMode::Off,
            pending_parsing_blocking_script: None,
            width: 800,
            height: 600,
        }
    }

    pub fn get_display_data(&self) -> DisplayBox {
        let mut root = DisplayBox::new();
        root.children = self.doctree.get_display_data(self);
        // TODO: Fix the relative positions of the nodes

        root
    }

    pub fn set_quirks_mode(&mut self, new_mode: QuirksMode) {
        self.quirksmode = new_mode;
    }

    pub fn get_pending_parse_blocking(&self) -> Option<()> {
        self.pending_parsing_blocking_script
    }

    pub fn set_window_dimensions(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    pub fn get_window_dimensions(&self) -> (u64, u64) {
        (self.width, self.height)
    }
}
