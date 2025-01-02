pub struct Data {
    value: String,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            value: "".to_string(),
        }
    }
}
