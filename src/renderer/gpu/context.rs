use std::sync::Arc;

use wgpu::*;
use winit::window::Window;

pub(crate) struct RenderContext {
    pub(crate) surface: Surface<'static>,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) config: SurfaceConfiguration,
}

impl RenderContext {
    pub(crate) fn new(window: Window) -> Self {
        let instance = Instance::default();
        let window = Arc::new(window);
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = pollster::block_on(
            instance.request_adapter(&RequestAdapterOptions::default()),
        )
        .expect("Failed to find an appropriate adapter. Maybe update your graphics drivers.");
        let (device, queue) =
            pollster::block_on(adapter.request_device(&DeviceDescriptor::default(), None)).unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let format = *surface_capabilities
            .formats
            .first()
            .unwrap_or(&TextureFormat::Rgba8Unorm);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
        }
    }
}
