use gfx::render_cmds::GFXRenderCommand;
use sunbeam_html;

pub struct Pane {
    id: usize,
    pub title: String,
    pub is_active: bool,
    pub document: Option<sunbeam_html::Document>,
    pub render_cmds: Vec<GFXRenderCommand>,

    // For now storing raw html string for debugability
    html_data: Option<String>,
    url: Option<String>,
}
