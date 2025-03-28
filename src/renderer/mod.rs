use winit::dpi::PhysicalSize;

pub mod gpu;

pub trait Renderer {
    fn render_frame(&self);
    fn resize(&mut self, size: PhysicalSize<u32>);
}
