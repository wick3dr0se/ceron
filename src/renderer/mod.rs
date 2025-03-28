pub mod gpu;

pub trait Renderer {
    fn render_frame(&mut self);
    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>);
}
