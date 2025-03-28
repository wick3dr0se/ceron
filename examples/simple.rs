use ceron::{app::App, renderer::gpu::GpuRenderer};

fn main() {
    let mut app = App::new(|window| GpuRenderer::new(window));

    app.run();
}
