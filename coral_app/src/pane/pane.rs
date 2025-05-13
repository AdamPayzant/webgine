use gfx::{GFXState, render_cmds::GFXRenderCommand};
use sunbeam_html::{
    self,
    display_data::display_box::{DisplayBox, DisplayBoxData},
    document,
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

    pub fn generate_render_cmds(&mut self, gfx_state: &GFXState) {
        // Clear the current render cmds
        self.render_cmds = Vec::new();
        let doc = match &self.document {
            Some(d) => d,
            None => {
                return;
            }
        };
        let dom = doc.get_display_data();

        fn internal_generate_cmds(
            display: &DisplayBox,
            offset: &[f32; 2],
        ) -> Vec<GFXRenderCommand> {
            let mut res = Vec::new();
            for c in &display.children {
                res.append(&mut internal_generate_cmds(c, offset));
            }

            // TODO: Implement this properly
            match &display.data {
                DisplayBoxData::Text(t) => {
                    res.push(GFXRenderCommand::Text {
                        position: offset.clone(),
                        content: t.data.clone(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                }
                // TODO: Finish the rest of the elements
                _ => {}
            };

            res
        }

        let mut offset = [0.0, 0.0];
        self.render_cmds = internal_generate_cmds(&dom, &offset);
    }
}
