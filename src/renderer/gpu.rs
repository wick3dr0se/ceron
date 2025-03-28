use pollster::block_on;
use wgpu::*;
use winit::{dpi::PhysicalSize, window::Window};

use crate::vertex::{INDICES, VERTICES, Vertex};

use super::Renderer;

pub struct GpuRenderer {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl GpuRenderer {
    pub fn new(window: Window) -> Self {
        let instance = Instance::default();

        let size = window.inner_size();
        let surface = instance.create_surface(window).unwrap();

        let adapter = block_on(instance.request_adapter(&RequestAdapterOptions::default()))
            .expect("Failed to find an appropriate adapter. Maybe update your graphics drivers.");
        let (device, queue) =
            block_on(adapter.request_device(&DeviceDescriptor::default(), None)).unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let format = *surface_capabilities
            .formats
            .first()
            .unwrap_or(&TextureFormat::Rgba8Unorm);
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let vertex_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: ShaderSource::Wgsl(include_str!("../../shaders/vertex.wgsl").into()),
        });
        let fragment_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: ShaderSource::Wgsl(include_str!("../../shaders/fragment.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &vertex_shader,
                entry_point: Some("main"),
                buffers: &[VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &Vertex::ATTRIBUTES,
                }],
                compilation_options: PipelineCompilationOptions::default(),
            },
            cache: None,
            fragment: Some(FragmentState {
                module: &fragment_shader,
                entry_point: Some("main"),
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });

        let vertex_buffer = util::DeviceExt::create_buffer_init(
            &device,
            &util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: BufferUsages::VERTEX,
            },
        );
        let index_buffer = util::DeviceExt::create_buffer_init(
            &device,
            &util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: BufferUsages::INDEX,
            },
        );

        Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            vertex_buffer,
            index_buffer,
        }
    }
}

impl Renderer for GpuRenderer {
    fn render_frame(&self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLUE),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&self.pipeline);

            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        }

        self.queue.submit([encoder.finish()]);
        output.present();
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;

        self.surface.configure(&self.device, &self.config);
    }
}
