use crate::html_elements::common_attributes;

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeOptions {
    Row,
    Col,
    Rowgroup,
    Colgroup,
}

impl ScopeOptions {
    pub fn derive_scope(value: &str) -> Option<ScopeOptions> {
        match value {
            "row" => Some(Self::Row),
            "col" => Some(Self::Col),
            "rowgroup" => Some(Self::Rowgroup),
            "colgroup" => Some(Self::Colgroup),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Th {
    abbr: Option<String>,
    colspan: u16,
    headers: Vec<String>, // ID
    rowspan: u16,
    scope: Option<ScopeOptions>,
}

impl Default for Th {
    fn default() -> Self {
        Th {
            abbr: None,
            colspan: 1,
            headers: Vec::new(),
            rowspan: 1,
            scope: None,
        }
    }
}

impl common_attributes::Element for Th {
    fn add_attribute(&mut self, name: String, value: String) {
        match name.as_str() {
            "abbr" => self.abbr = Some(value),
            "colspan" => {
                self.colspan = match value.parse() {
                    Ok(v) => {
                        if v < 1000 {
                            v
                        } else {
                            1
                        }
                    }
                    Err(_) => 1,
                }
            }
            "headers" => self.headers = value.split(" ").map(|s| s.to_string()).collect(),
            "rowspan" => {
                self.rowspan = match value.parse::<usize>() {
                    Ok(v) => {
                        if v > 65534 {
                            65534
                        } else {
                            v as u16
                        }
                    }
                    Err(_) => 1,
                }
            }
            "scope" => self.scope = ScopeOptions::derive_scope(value.as_str()),
            _ => {}
        }
    }
}
