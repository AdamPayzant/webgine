use gfx::{GFXState, render_cmds::GFXRenderCommand, utils};
use sunbeam_html::{
    self,
    display_data::display_box::{DisplayBox, DisplayBoxData},
};

pub struct Pane {
    pub title: String,
    pub document: Option<sunbeam_html::Document>,
    pub render_cmds: Vec<GFXRenderCommand>,

    // For now storing raw html string for debugability
    html_data: Option<String>,
    url: Option<String>,
}

impl Pane {
    pub fn new_from_file(filepath: &str) -> Pane {
        use std::fs;
        use std::path::PathBuf;
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_resources/basic.html");

        let doc = sunbeam_html::parse_document(&fs::read_to_string(d).unwrap());

        Pane {
            title: "basic".to_string(),
            document: Some(doc),
            render_cmds: Vec::new(),
            html_data: None,
            url: None,
        }
    }

    pub fn generate_render_cmds(&mut self, gfx_state: &mut GFXState) {
        // Clear the current render cmds
        self.render_cmds = Vec::new();
        let doc = match &self.document {
            Some(d) => d,
            None => {
                return;
            }
        };
        let dom = doc.get_display_data();

        let mut parents: Vec<(&DisplayBox, usize)> = Vec::new();
        let mut active_styling: Vec<&sunbeam_html::display_data::styling::Styling> = Vec::new();

        parents.push((&dom, 0));
        let mut offset = [0.0, 0.0];
        while !parents.is_empty() {
            if let Some((node, idx)) = parents.last_mut() {
                if node.children.len() <= *idx {
                    // Generate render command for this node
                    match &node.data {
                        DisplayBoxData::Text(t) => {
                            let str = t.data.trim().to_string();
                            if !str.is_empty() {
                                let (font, size) = gfx_state.get_text_default_font_and_winsize();
                                let buffer = utils::create_default_text_buffer(&str, font, size);

                                let cmd = GFXRenderCommand::Text {
                                    position: offset.clone(),
                                    content: buffer,
                                    color: [0.0, 0.0, 0.0, 1.0],
                                };

                                let (x, y) = utils::get_size_of_text(&cmd);
                                offset[0] = 0.;
                                offset[1] += y;

                                self.render_cmds.push(cmd);
                            }
                        }
                        DisplayBoxData::None => {}
                        // TODO: Finish the rest of the elements
                        _ => {}
                    }

                    parents.pop();
                    active_styling.pop();
                } else {
                    let child = &node.children[*idx];

                    *idx += 1;
                    active_styling.push(&node.style);
                    parents.push((child, 0));
                }
            }
        }
    }
}
