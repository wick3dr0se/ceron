use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

impl Vertex {
    pub const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2,  // position
        1 => Float32x4   // color
    ];

    const fn new(x: f32, y: f32, r: f32, g: f32, b: f32, a: f32) -> Self {
        Vertex {
            position: [x, y],
            color: [r, g, b, a],
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex::new(-0.5, -0.5, 1.0, 0.0, 0.0, 1.0), // bottom left/red
    Vertex::new(0.5, -0.5, 0.0, 1.0, 0.0, 1.0),  // bottom right/green
    Vertex::new(0.0, 0.5, 0.0, 0.0, 1.0, 1.0),   // top center/blue
];
pub const INDICES: &[u16] = &[0, 1, 2];
