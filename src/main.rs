use sdl2;

#[allow(dead_code)]
mod core;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let gl: glow::Context;
    let mut window: core::window::Window;
    (gl, window) = core::window::Window::new(&sdl_context, 800, 600, "test", false);
    let mut projection = core::projection::Projection::new(
        70.0,
        window.get_width() as f32,
        window.get_height() as f32,
        0.1,
        1000.0,
    );
    let mut engine = core::engine::Engine::new(&sdl_context, window, &gl, &projection);

    engine.start();
}
