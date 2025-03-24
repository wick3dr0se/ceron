use bytemuck::{Pod, Zeroable};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

impl Vertex {
    pub const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2,  // position
        1 => Float32x4   // color
    ];
}

pub const TRIANGLE_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, -0.5],      // bottom left
        color: [1.0, 0.0, 0.0, 1.0], // red
    },
    Vertex {
        position: [0.5, -0.5],       // bottom right
        color: [0.0, 1.0, 0.0, 1.0], // green
    },
    Vertex {
        position: [0.0, 0.5],        // top center
        color: [0.0, 0.0, 1.0, 1.0], // blue
    },
];

pub const TRIANGLE_INDICES: &[u16] = &[0, 1, 2];

pub const QUAD_VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, -0.5],      // bottom left
        color: [1.0, 0.0, 0.0, 1.0], // red
    },
    Vertex {
        position: [0.5, -0.5],       // bottom right
        color: [0.0, 1.0, 0.0, 1.0], // green
    },
    Vertex {
        position: [0.5, 0.5],        // top right
        color: [0.0, 0.0, 1.0, 1.0], // blue
    },
    Vertex {
        position: [-0.5, 0.5],       // top left
        color: [1.0, 1.0, 0.0, 1.0], // yellow
    },
];

pub const QUAD_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];
