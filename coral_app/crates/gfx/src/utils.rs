use glyphon;

use crate::GFXRenderCommand;
use crate::vertex::Vertex;

pub const VERTEX_BUFFER_MAX_RECT: u64 = 10;

pub fn compute_vertex_buffer_offset(offset: u64) -> u64 {
    offset * std::mem::size_of::<Vertex>() as u64 * 6
}

pub fn new_vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Vertex Buffer"),
        size: (std::mem::size_of::<Vertex>() as u64 * 6) * VERTEX_BUFFER_MAX_RECT,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

pub fn new_text_buffer(
    font_system: &mut glyphon::FontSystem,
    metrics: glyphon::Metrics,
) -> glyphon::Buffer {
    glyphon::Buffer::new(font_system, metrics)
}

pub fn float_colors_to_glyphon_rgba(colors: [f32; 4]) -> glyphon::Color {
    glyphon::Color::rgba(
        (colors[0] * 255.0) as u8,
        (colors[1] * 255.0) as u8,
        (colors[2] * 255.0) as u8,
        (colors[3] * 255.0) as u8,
    )
}

pub fn clip_space_to_absolute(pos: [f32; 2], window_width: u32, window_height: u32) -> [f32; 2] {
    [
        ((pos[0] + 1.0) / 2.0) * window_width as f32,
        ((pos[1] + 1.0) / 2.0) * window_height as f32,
    ]
}

pub fn absolute_to_clip_space(pos: [f32; 2]) -> [f32; 2] {
    [(pos[0] * 2.0 - 1.0), (1.0 - pos[1] * 2.0)]
}

pub fn create_rect_vertices(pos: [f32; 2], size: [f32; 2], color: [f32; 4]) -> [Vertex; 6] {
    let [x, y] = pos;
    let [w, h] = size;

    [
        Vertex {
            position: [x, y],
            color,
        },
        Vertex {
            position: [x, y + h],
            color,
        },
        Vertex {
            position: [x + w, y],
            color,
        },
        Vertex {
            position: [x + w, y],
            color,
        },
        Vertex {
            position: [x, y + h],
            color,
        },
        Vertex {
            position: [x + w, y + h],
            color,
        },
    ]
}

pub fn create_outline(
    pos: [f32; 2],
    size: [f32; 2],
    thickness: f32,
    window_width: u32,
    window_height: u32,
    color: [f32; 4],
) -> [[Vertex; 6]; 4] {
    let [x, y] = pos;
    let [w, h] = size;

    let thickness_x = (thickness / window_width as f32) * 2.0;
    let thickness_y = (thickness / window_height as f32) * 2.0;

    [
        [
            Vertex {
                position: [x, y],
                color,
            },
            Vertex {
                position: [x, y + thickness_y],
                color,
            },
            Vertex {
                position: [x + w, y],
                color,
            },
            Vertex {
                position: [x + w, y],
                color,
            },
            Vertex {
                position: [x, y + thickness_y],
                color,
            },
            Vertex {
                position: [x + w, y + thickness_y],
                color,
            },
        ],
        [
            Vertex {
                position: [x + w + thickness_x, y],
                color,
            },
            Vertex {
                position: [x + w + thickness_x, y + h + thickness_y],
                color,
            },
            Vertex {
                position: [x + w, y],
                color,
            },
            Vertex {
                position: [x + w, y],
                color,
            },
            Vertex {
                position: [x + w + thickness_x, y + h + thickness_y],
                color,
            },
            Vertex {
                position: [x + w, y + h + thickness_y],
                color,
            },
        ],
        [
            Vertex {
                position: [x + thickness_x, y],
                color,
            },
            Vertex {
                position: [x + thickness_x, y + h],
                color,
            },
            Vertex {
                position: [x, y],
                color,
            },
            Vertex {
                position: [x, y],
                color,
            },
            Vertex {
                position: [x + thickness_x, y + h],
                color,
            },
            Vertex {
                position: [x, y + h],
                color,
            },
        ],
        [
            Vertex {
                position: [x, y + h],
                color,
            },
            Vertex {
                position: [x, y + h + thickness_y],
                color,
            },
            Vertex {
                position: [x + w, y + h],
                color,
            },
            Vertex {
                position: [x + w, y + h],
                color,
            },
            Vertex {
                position: [x, y + h + thickness_y],
                color,
            },
            Vertex {
                position: [x + w, y + h + thickness_y],
                color,
            },
        ],
    ]
}

pub fn create_default_text_buffer(
    content: &str,
    font_system: &mut glyphon::FontSystem,
    size: [f64; 2],
) -> glyphon::Buffer {
    let mut buf = glyphon::Buffer::new(font_system, glyphon::Metrics::new(30.0, 42.0));
    buf.set_size(font_system, Some(size[0] as f32), Some(size[1] as f32));

    buf.set_text(
        font_system,
        &content,
        glyphon::Attrs::new().family(glyphon::Family::SansSerif),
        glyphon::Shaping::Advanced,
    );
    buf.shape_until_scroll(font_system, true);

    buf
}

pub fn get_size_of_text(cmd: &GFXRenderCommand) -> (f32, f32) {
    let text_buffer = match cmd {
        GFXRenderCommand::Text {
            position,
            content,
            color,
        } => content,
        _ => return (0.0, 0.0),
    };

    let line_height = text_buffer.lines.len() as f32 * text_buffer.metrics().line_height;
    let layout_runs = text_buffer.layout_runs();
    let mut run_width: f32 = 0.;
    for run in layout_runs {
        run_width = run_width.max(run.line_w);
    }

    (run_width, line_height)
}
