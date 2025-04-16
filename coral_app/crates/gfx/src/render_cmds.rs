#[derive(Clone, Debug)]
pub enum GFXRenderCommand {
    Rect {
        position: [f32; 2],
        size: [f32; 2],
        color: [f32; 4],
    },
    Outline {
        position: [f32; 2],
        size: [f32; 2],
        thickness: f32,
        color: [f32; 4],
    },
    Text {
        position: [f32; 2],
        content: String,
        color: [f32; 4],
    },
}
