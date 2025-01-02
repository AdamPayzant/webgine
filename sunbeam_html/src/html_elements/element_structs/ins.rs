pub struct Ins {
    cite: Option<String>,
    datetime: Option<String>, // Datetime string
}

impl Default for Ins {
    fn default() -> Self {
        Ins {
            cite: None,
            datetime: None,
        }
    }
}
