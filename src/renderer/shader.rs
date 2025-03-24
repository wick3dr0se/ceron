use std::fs::read_to_string;

pub fn load_shader(device: &wgpu::Device, path: &str) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(path),
        source: wgpu::ShaderSource::Wgsl(
            read_to_string(format!("shaders/{}", path)).unwrap().into(),
        ),
    })
}
