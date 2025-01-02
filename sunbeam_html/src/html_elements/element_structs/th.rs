pub enum ScopeOptions {
    Row,
    Col,
    Rowgroup,
    Colgroup,
}

pub struct Th {
    abbr: Option<String>,
    colspan: u16,
    headers: Vec<String>, // ID
    rowspan: usize,
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
