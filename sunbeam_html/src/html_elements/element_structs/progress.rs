pub struct Progress {
    max: f64,
    value: Option<f64>,
}

impl Default for Progress {
    fn default() -> Self {
        Progress {
            max: 1.0,
            value: None,
        }
    }
}
