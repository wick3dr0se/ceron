use ceron::{
    Vec2,
    app::App,
    renderer::gpu::{Color, GpuRenderer},
};

fn main() {
    let size = 32.0;
    let pos = Vec2::splat(size);

    let mut app = App::new(move |window| {
        let mut renderer = GpuRenderer::new(window);

        renderer.set_background(Color::GREEN);

        renderer.draw_triangle(pos, size, Color::WHITE);
        renderer.draw_triangle(pos * 2.0, size, Color::BLACK);
        renderer.draw_triangle(pos * 3.0, size, Color::WHITE);

        renderer
    });

    app.run();
}
