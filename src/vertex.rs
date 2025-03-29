use bytemuck::{Pod, Zeroable};
use wgpu::Color;

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

    pub const fn new(x: f32, y: f32, color: Color) -> Self {
        Vertex {
            position: [x, y],
            color: [
                color.r as f32,
                color.g as f32,
                color.b as f32,
                color.a as f32,
            ],
        }
    }
}
