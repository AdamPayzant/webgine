pub struct Map {
    name: Option<String>,
}

impl Default for Map {
    fn default() -> Self {
        Map { name: None }
    }
}
