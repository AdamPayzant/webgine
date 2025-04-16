use glyphon;

use crate::vertex::Vertex;

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
) -> [[Vertex; 6]; 3] {
    let [x, y] = pos;
    let [w, h] = size;

    let mut color2 = [1.0, 1.0, 0.0, 0.0];
    let mut color3 = [1.0, 0.0, 1.0, 0.0];
    let mut color4 = [1.0, 0.0, 0.0, 1.0];

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
                color: color2,
            },
            Vertex {
                position: [x + w + thickness, y + h],
                color: color2,
            },
            Vertex {
                position: [x + w, y],
                color: color2,
            },
            Vertex {
                position: [x + w, y],
                color: color2,
            },
            Vertex {
                position: [x + w + thickness, y + h],
                color: color2,
            },
            Vertex {
                position: [x + w, y + h],
                color: color2,
            },
        ],
        [
            Vertex {
                position: [x + thickness, y],
                color: color3,
            },
            Vertex {
                position: [x + thickness, y + h],
                color: color3,
            },
            Vertex {
                position: [x, y],
                color: color3,
            },
            Vertex {
                position: [x, y],
                color: color3,
            },
            Vertex {
                position: [x + thickness, y + h],
                color: color3,
            },
            Vertex {
                position: [x, y + h],
                color: color3,
            },
        ],
        // [
        //     Vertex {
        //         position: [x, y + h],
        //         color: color4,
        //     },
        //     Vertex {
        //         position: [x, y + h + thickness],
        //         color: color4,
        //     },
        //     Vertex {
        //         position: [x + w, y + h],
        //         color: color4,
        //     },
        //     Vertex {
        //         position: [x + w, y + h],
        //         color: color4,
        //     },
        //     Vertex {
        //         position: [x, y + h + thickness],
        //         color: color4,
        //     },
        //     Vertex {
        //         position: [x + w, y + h + thickness],
        //         color: color4,
        //     },
        // ],
    ]
}
