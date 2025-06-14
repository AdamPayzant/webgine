use gfx;

pub struct HeaderState {
    tab_list: Vec<(usize, String)>,
}

impl HeaderState {
    pub fn new() -> HeaderState {
        Self {
            tab_list: Vec::new(),
        }
    }

    pub fn generate_render_cmds() -> Vec<gfx::render_cmds::GFXRenderCommand> {
        Vec::new()
    }
}
