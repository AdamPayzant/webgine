pub struct Object {
    data: Option<String>, // URL
    form: Option<String>, // ID
    height: Option<usize>,
    name: Option<String>,
    content_type: Option<String>,
    width: Option<usize>,
}

impl Default for Object {
    fn default() -> Self {
        Object {
            data: None,
            form: None,
            height: None,
            name: None,
            content_type: None,
            width: None,
        }
    }
}
