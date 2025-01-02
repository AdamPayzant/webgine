pub struct Html {
    xmlns: Option<String>, // URL
}

impl Default for Html {
    fn default() -> Self {
        Html { xmlns: None }
    }
}
