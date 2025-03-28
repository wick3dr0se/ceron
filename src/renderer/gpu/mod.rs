mod context;
mod pipeline;
mod shader;

use pipeline::Pipeline;
use wgpu::*;
use winit::{dpi::PhysicalSize, window::Window};

use context::RenderContext;

use crate::{renderer::Renderer, vertex::INDICES};

pub struct GpuRenderer {
    ctx: RenderContext,
    pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl GpuRenderer {
    pub fn new(window: Window) -> Self {
        let ctx = RenderContext::new(window);
        let (pipeline, vertex_buffer, index_buffer) = Pipeline::new(&ctx);

        Self {
            ctx,
            pipeline,
            vertex_buffer,
            index_buffer,
        }
    }
}

impl Renderer for GpuRenderer {
    fn render_frame(&self) {
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
            pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
            pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
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
