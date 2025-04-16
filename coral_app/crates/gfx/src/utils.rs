use glyphon;

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

pub fn float_colors_to_glyphon_rgba(colors: [f32; 4]) -> glyphon::Color {
    glyphon::Color::rgba(
        (colors[0] * 255.0) as u8,
        (colors[1] * 255.0) as u8,
        (colors[2] * 255.0) as u8,
        (colors[3] * 255.0) as u8,
    )
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
    color: [f32; 4],
) -> [[Vertex; 6]; 4] {
    let [x, y] = pos;
    let [w, h] = size;

    [
        [
            Vertex {
                position: [x, y],
                color,
            },
            Vertex {
                position: [x, y + thickness],
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
                position: [x, y + thickness],
                color,
            },
            Vertex {
                position: [x + w, y + thickness],
                color,
            },
        ],
        [
            Vertex {
                position: [x + w + thickness, y],
                color,
            },
            Vertex {
                position: [x + w + thickness, y + h],
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
                position: [x + w + thickness, y + h],
                color,
            },
            Vertex {
                position: [x + w, y + h],
                color,
            },
        ],
        [
            Vertex {
                position: [x + thickness, y],
                color,
            },
            Vertex {
                position: [x + thickness, y + h],
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
                position: [x + thickness, y + h],
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
                position: [x, y + h + thickness],
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
                position: [x, y + h + thickness],
                color,
            },
            Vertex {
                position: [x + w, y + h + thickness],
                color,
            },
        ],
    ]
}
