pub struct Del {
    cite: Option<String>,
    datetime: Option<String>, // TODO: Probably should implement an explicit date-string type
}

impl Default for Del {
    fn default() -> Self {
        Del {
            cite: None,
            datetime: None,
        }
    }
}
