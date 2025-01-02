pub struct OptGroup {
    disabled: bool,
    label: String,
}

impl Default for OptGroup {
    fn default() -> Self {
        OptGroup {
            disabled: false,
            label: "".to_string(),
        }
    }
}
