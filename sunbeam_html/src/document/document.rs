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
}

impl Document {
    pub fn new() -> Document {
        Document {
            doctree: doctree::Doctree::new(),
            doctype: "".to_string(),
            quirksmode: QuirksMode::Off,
            pending_parsing_blocking_script: None,
        }
    }

    pub fn set_quirks_mode(&mut self, new_mode: QuirksMode) {
        self.quirksmode = new_mode;
    }

    pub fn get_pending_parse_blocking(&self) -> Option<()> {
        self.pending_parsing_blocking_script
    }
}
