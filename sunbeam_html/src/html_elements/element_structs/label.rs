pub struct Label {
    label_for: Option<String>, // ID
}

impl Default for Label {
    fn default() -> Self {
        Label { label_for: None }
    }
}
