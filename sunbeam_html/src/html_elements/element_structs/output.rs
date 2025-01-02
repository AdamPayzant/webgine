pub struct Output {
    output_for: Vec<String>,
    form: Option<String>, // ID
    name: Option<String>,
}

impl Default for Output {
    fn default() -> Self {
        Output {
            output_for: Vec::new(),
            form: None,
            name: None,
        }
    }
}
