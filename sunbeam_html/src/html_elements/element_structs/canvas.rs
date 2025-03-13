use crate::html_elements::common_attributes;

#[derive(Clone)]
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

impl common_attributes::Element for Canvas {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "height" => {
                self.height = match value.parse::<usize>() {
                    Ok(h) => h,
                    Err(_) => return,
                }
            }
            "width" => {
                self.width = match value.parse::<usize>() {
                    Ok(w) => w,
                    Err(_) => return,
                }
            }
            _ => {}
        }
    }
}
