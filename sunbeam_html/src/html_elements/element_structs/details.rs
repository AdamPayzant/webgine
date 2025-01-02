pub struct Details {
    open: bool,
    name: Option<String>,
}

impl Default for Details {
    fn default() -> Self {
        Details {
            open: false,
            name: None,
        }
    }
}
