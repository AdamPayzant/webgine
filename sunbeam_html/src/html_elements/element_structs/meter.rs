pub struct Meter {
    value: f64,
    min: f64,
    max: f64,
    low: f64,
    high: f64,
    optimum: Option<f64>,
    form: Option<String>, // ID
}

impl Default for Meter {
    fn default() -> Self {
        Meter {
            value: 0.0,
            min: 0.0,
            max: 1.0,
            low: 0.0,
            high: 1.0,
            optimum: None,
            form: None,
        }
    }
}
