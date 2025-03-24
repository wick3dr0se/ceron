use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::Window,
};

use crate::renderer::{Renderer, context::RenderContext};

pub use wgpu::Color;

pub trait AppHandler {
    fn update(&mut self, ctx: &mut RenderContext);
}

pub struct App<H: AppHandler> {
    window: Option<Window>,
    renderer: Option<Renderer>,
    handler: H,
}

impl<H: AppHandler> ApplicationHandler for App<H> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("Ceron"))
            .unwrap();

        self.renderer = Some(Renderer::new(&window));

        self.window = Some(window);
        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                if let (Some(renderer), Some(window)) = (&mut self.renderer, &self.window) {
                    renderer.render(window, |ctx| self.handler.update(ctx));
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size);
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

pub fn run<H: AppHandler + 'static>(handler: H) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App {
        window: None,
        renderer: None,
        handler,
    };

    event_loop.run_app(&mut app).unwrap();
}
