pub struct RenderContext<'a> {
    pub(crate) encoder: &'a mut wgpu::CommandEncoder,
    pub(crate) view: &'a wgpu::TextureView,
}

impl<'a> RenderContext<'a> {
    pub fn clear(&mut self, color: wgpu::Color) {
        let _render_pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: self.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
    }
}
