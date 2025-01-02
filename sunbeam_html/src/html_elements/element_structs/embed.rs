pub struct Embed {
    height: usize,
    width: usize,
    src: Option<String>,       // URL
    mime_type: Option<String>, // Mime type
}

impl Default for Embed {
    fn default() -> Self {
        Embed {
            height: 0,
            width: 0,
            src: None,
            mime_type: None,
        }
    }
}
