use ceron::{
    app::{AppHandler, Color, run},
    renderer::context::RenderContext,
};

struct MyApp;

impl AppHandler for MyApp {
    fn update(&mut self, ctx: &mut RenderContext) {
        ctx.clear(Color::BLACK);
    }
}

fn main() {
    run(MyApp);
}
