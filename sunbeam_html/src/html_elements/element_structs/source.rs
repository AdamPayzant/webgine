pub struct Source {
    mime_type: Option<String>, // TODO: Change when I have mime type system
    src: Option<String>,       // URL
    srcset: Vec<String>,       // List of URLs
    sizes: Vec<String>,        // TODO: Need a type for sizes
    media: Option<String>,     // Need a type for media queries
    height: Option<usize>,
    width: Option<usize>,
}

impl Default for Source {
    fn default() -> Self {
        Source {
            mime_type: None,
            src: None,
            srcset: Vec::new(),
            sizes: Vec::new(),
            media: None,
            height: None,
            width: None,
        }
    }
}
