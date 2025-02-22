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
}

impl Document {
    pub fn new() -> Document {
        Document {
            doctree: doctree::Doctree::new(),
            doctype: "".to_string(),
            quirksmode: QuirksMode::Off,
        }
    }

    pub fn set_quirks_mode(&mut self, new_mode: QuirksMode) {
        self.quirksmode = new_mode;
    }
}
