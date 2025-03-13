use crate::html_elements::common_attributes;

#[derive(Clone)]
pub struct Td {
    colspan: u16,
    headers: Vec<String>, // List of IDs
    rowspan: u32,
}

impl Default for Td {
    fn default() -> Self {
        Td {
            colspan: 1,
            headers: Vec::new(),
            rowspan: 1,
        }
    }
}

impl common_attributes::Element for Td {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "colspan" => {
                self.colspan = match value.parse() {
                    Ok(v) => v,
                    Err(_) => 1,
                }
            }
            "headers" => self.headers = value.split(" ").map(|s| s.to_string()).collect(),
            "rowspan" => match value.parse::<usize>() {
                Ok(v) => self.rowspan = if v < 65534 { v as u32 } else { 65534 },
                Err(_) => {}
            },
            _ => {}
        }
    }
}
