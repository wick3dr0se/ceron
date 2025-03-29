mod context;
mod pipeline;
mod shader;

use glam::{Vec2, vec2};
use wgpu::{util::DeviceExt, *};
use winit::{dpi::PhysicalSize, window::Window};

use context::RenderContext;
use pipeline::Pipeline;

use crate::{renderer::Renderer, vertex::Vertex};

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

    pub fn surface_size(&self) -> Vec2 {
        vec2(self.ctx.config.width as f32, self.ctx.config.height as f32)
    }

    pub fn set_background(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn draw_line(&mut self, start: Vec2, end: Vec2, width: f32, color: Color) {
        let surface_size = self.surface_size();

        let start_ndc = Vec2::new(
            (start.x / surface_size.x) * 2.0 - 1.0,
            -(start.y / surface_size.y) * 2.0 + 1.0,
        );
        let end_ndc = Vec2::new(
            (end.x / surface_size.x) * 2.0 - 1.0,
            -(end.y / surface_size.y) * 2.0 + 1.0,
        );

        let dir = (end_ndc - start_ndc).normalize();
        let perp = vec2(-dir.y, dir.x) * (width / surface_size.x * 2.0);

        let vertices = vec![
            Vertex::new(start_ndc.x - perp.x, start_ndc.y - perp.y, color),
            Vertex::new(start_ndc.x + perp.x, start_ndc.y + perp.y, color),
            Vertex::new(end_ndc.x - perp.x, end_ndc.y - perp.y, color),
            Vertex::new(end_ndc.x + perp.x, end_ndc.y + perp.y, color),
        ];
        let indices = vec![0, 1, 2, 1, 3, 2];

        self.draw_commands.push(DrawCommand { vertices, indices });
    }

    pub fn draw_triangle(&mut self, pos: Vec2, size: f32, color: Color) {
        let window_size = vec2(self.ctx.config.width as f32, self.ctx.config.height as f32);
        let pos_ndc = Vec2::new(
            (pos.x / window_size.x) * 2.0 - 1.0,
            -(pos.y / window_size.y) * 2.0 + 1.0,
        );
        let size_ndc = size / window_size.x as f32;

        let vertices = vec![
            Vertex::new(pos_ndc.x, pos_ndc.y + size_ndc, color),
            Vertex::new(pos_ndc.x - size_ndc, pos_ndc.y - size_ndc, color),
            Vertex::new(pos_ndc.x + size_ndc, pos_ndc.y - size_ndc, color),
        ];
        let indices = vec![0, 1, 2];

        self.draw_commands.push(DrawCommand { vertices, indices });
    }

    pub fn draw_rectangle(&mut self, pos: Vec2, size: Vec2, color: Color) {
        let window_size = vec2(self.ctx.config.width as f32, self.ctx.config.height as f32);
        let pos_ndc = Vec2::new(
            (pos.x / window_size.x) * 2.0 - 1.0,
            -(pos.y / window_size.y) * 2.0 + 1.0,
        );
        let size_ndc = (size / window_size) * 2.0;
        let half = size_ndc * 0.5;

        let vertices = vec![
            Vertex::new(pos_ndc.x - half.x, pos_ndc.y + half.y, color),
            Vertex::new(pos_ndc.x + half.x, pos_ndc.y + half.y, color),
            Vertex::new(pos_ndc.x - half.x, pos_ndc.y - half.y, color),
            Vertex::new(pos_ndc.x + half.x, pos_ndc.y - half.y, color),
        ];
        let indices = vec![0, 1, 2, 1, 3, 2];

        self.draw_commands.push(DrawCommand { vertices, indices });
    }

    pub fn draw_circle(&mut self, pos: Vec2, radius: f32, color: Color, segments: u16) {
        let window_size = vec2(self.ctx.config.width as f32, self.ctx.config.height as f32);

        let pos_ndc = Vec2::new(
            (pos.x / window_size.x) * 2.0 - 1.0,
            -(pos.y / window_size.y) * 2.0 + 1.0,
        );
        let radius_ndc = radius / window_size.x;

        let mut vertices = vec![Vertex::new(pos_ndc.x, pos_ndc.y, color)];
        let mut indices = Vec::new();

        for i in 0..=segments {
            let theta = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let x = pos_ndc.x + radius_ndc * theta.cos();
            let y = pos_ndc.y + radius_ndc * theta.sin();
            vertices.push(Vertex::new(x, y, color));

            if i > 0 {
                indices.push(0);
                indices.push(i);
                indices.push(i + 1);
            }
        }

        indices.push(0);
        indices.push(segments.try_into().unwrap());
        indices.push(1);

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
                pass.draw_indexed(0..cmd.indices.len() as u32, 0, 0..1);
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
