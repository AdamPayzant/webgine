use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
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

impl common_attributes::Element for Progress {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "max" => match value.parse() {
                Ok(v) => self.max = v,
                Err(_) => {}
            },
            "value" => {
                self.value = match value.parse() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            }
            _ => {}
        }
    }
}
