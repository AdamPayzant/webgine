pub struct Time {
    datetime: String, // TODO: Implement date string type
}

impl Default for Time {
    fn default() -> Self {
        Time {
            datetime: "".to_string(),
        }
    }
}
