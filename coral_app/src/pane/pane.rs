use sunbeam_html;

pub struct Pane {
    pub title: String,
    pub is_active: bool,
    pub document: sunbeam_html::Document,
}
