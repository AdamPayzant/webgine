pub struct Canvas {
    height: usize,
    width: usize,
}

impl Default for Canvas {
    fn default() -> Self {
        Canvas {
            height: 100,
            width: 320,
        }
    }
}
