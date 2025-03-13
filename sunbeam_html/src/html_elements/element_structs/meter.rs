use crate::html_elements::common_attributes;

#[derive(Clone)]
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

impl common_attributes::Element for Meter {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "value" => match value.parse() {
                Ok(v) => self.value = v,
                Err(_) => {}
            },
            "min" => match value.parse() {
                Ok(v) => self.min = v,
                Err(_) => {}
            },
            "max" => match value.parse() {
                Ok(v) => self.max = v,
                Err(_) => {}
            },
            "low" => match value.parse() {
                Ok(v) => self.low = v,
                Err(_) => {}
            },
            "high" => match value.parse() {
                Ok(v) => self.high = v,
                Err(_) => {}
            },
            "optimum" => {
                self.optimum = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            }
            "form" => self.form = Some(value),
            _ => {}
        }
    }
}
