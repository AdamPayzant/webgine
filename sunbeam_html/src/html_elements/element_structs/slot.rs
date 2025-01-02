pub struct Slot {
    name: Option<String>,
}

impl Default for Slot {
    fn default() -> Self {
        Slot { name: None }
    }
}
