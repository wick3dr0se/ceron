mod context;
mod pipeline;
mod shader;

use glam::{Vec2, vec2};
use wgpu::{util::DeviceExt, *};
use winit::{dpi::PhysicalSize, window::Window};

use context::RenderContext;
use pipeline::Pipeline;

use crate::{
    renderer::Renderer,
    vertex::{TRI_INDICES, Vertex},
};

pub use wgpu::Color;

pub struct DrawCommand {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

pub struct GpuRenderer {
    ctx: RenderContext,
    pipeline: RenderPipeline,
    background_color: Color,
    draw_commands: Vec<DrawCommand>,
}

impl GpuRenderer {
    pub fn new(window: Window) -> Self {
        let ctx = RenderContext::new(window);
        let pipeline = Pipeline::new(&ctx);

        Self {
            ctx,
            pipeline,
            draw_commands: Vec::new(),
            background_color: Color::WHITE,
        }
    }

    pub fn set_background(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn draw_triangle(&mut self, pos: Vec2, size: f32, color: Color) {
        let window_size = vec2(self.ctx.config.width as f32, self.ctx.config.height as f32);
        let pos_ndc = Vec2::new(
            (pos.x / window_size.x) * 2.0 - 1.0,
            -(pos.y / window_size.y) * 2.0 + 1.0,
        );
        let size_ndc = size / window_size.x as f32 * 2.0;

        let vertices = vec![
            Vertex::new(pos_ndc.x, pos_ndc.y + size_ndc, color),
            Vertex::new(pos_ndc.x - size_ndc, pos_ndc.y - size_ndc, color),
            Vertex::new(pos_ndc.x + size_ndc, pos_ndc.y - size_ndc, color),
        ];
        let indices = TRI_INDICES.into();

        self.draw_commands.push(DrawCommand { vertices, indices });
    }
}

impl Renderer for GpuRenderer {
    fn render_frame(&mut self) {
        let output = self.ctx.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(self.background_color),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&self.pipeline);

            for cmd in &self.draw_commands {
                let vertex_buffer =
                    self.ctx
                        .device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Vertex Buffer"),
                            contents: bytemuck::cast_slice(&cmd.vertices),
                            usage: BufferUsages::VERTEX,
                        });
                let index_buffer = util::DeviceExt::create_buffer_init(
                    &self.ctx.device,
                    &util::BufferInitDescriptor {
                        label: Some("Index Buffer"),
                        contents: bytemuck::cast_slice(&cmd.indices),
                        usage: BufferUsages::INDEX,
                    },
                );

                pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
                pass.draw_indexed(0..3, 0, 0..1);
            }
        }

        self.ctx.queue.submit([encoder.finish()]);
        output.present();
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.ctx.config.width = size.width;
        self.ctx.config.height = size.height;

        self.ctx
            .surface
            .configure(&self.ctx.device, &self.ctx.config);
    }
}
