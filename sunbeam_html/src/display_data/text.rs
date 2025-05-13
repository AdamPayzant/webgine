pub struct Text {
    pub data: String,
    pub font: Option<String>,
}

impl Text {
    pub fn get_dimensions(&self) -> (f32, f32) {
        (0.0, 0.0)
    }
}
