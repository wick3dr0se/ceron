use std::fs::read_to_string;

use wgpu::{ShaderModule, ShaderModuleDescriptor, ShaderSource};

use super::context::RenderContext;

pub(crate) struct Shader;

impl Shader {
    pub(crate) fn load(ctx: &RenderContext, path: &str) -> ShaderModule {
        let shader_path = read_to_string(format!("shaders/{}", path)).unwrap();

        ctx.device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(shader_path.into()),
        })
    }
}
