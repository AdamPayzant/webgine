pub struct Blockquote {
    cite: Option<String>,
}

impl Default for Blockquote {
    fn default() -> Self {
        Blockquote { cite: None }
    }
}
