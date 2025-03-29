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

        let surface_center = renderer.surface_size() / 2.0;

        renderer.draw_triangle(pos, size, Color::WHITE);
        renderer.draw_triangle(pos * 2.0, size, Color::BLACK);
        renderer.draw_rectangle(pos * 3.0, pos, Color::WHITE);
        renderer.draw_rectangle(pos * 4.0, pos, Color::BLACK);
        renderer.draw_line(pos * 4.0, pos * 5.0, 2.0, Color::RED);
        renderer.draw_circle(pos * 5.0, size, Color::BLUE, 20);
        renderer.draw_circle(surface_center, size * 8.0, Color::RED, 100);

        renderer
    });

    app.run();
}
