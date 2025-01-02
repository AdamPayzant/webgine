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
