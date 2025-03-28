use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

use crate::renderer::Renderer;

pub struct App<R: Renderer> {
    window: Option<Window>,
    renderer: Option<R>,
    constructor: Option<Box<dyn FnOnce(Window) -> R>>,
}

impl<R: Renderer> App<R> {
    pub fn new(constructor: impl FnOnce(Window) -> R + 'static) -> Self {
        Self {
            window: None,
            renderer: None,
            constructor: Some(Box::new(constructor)),
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();

        event_loop.run_app(self).unwrap();
    }
}

impl<R: Renderer> ApplicationHandler for App<R> {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let (Some(window), Some(renderer)) = (&self.window, &mut self.renderer) {
                    renderer.resize(size);
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.render_frame();
                }
            }
            _ => (),
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        self.window = Some(window);

        if let Some(window) = self.window.take() {
            let constructor = self.constructor.take().unwrap();
            self.renderer = Some(constructor(window));
        }
    }
}
