use sdl2;

#[allow(dead_code)]
mod core;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let gl: glow::Context;
    let mut window: core::window::Window;
    (gl, window) = core::window::Window::new(&sdl_context, 800, 600, "test", false);
    let mut engine = core::engine::Engine::new(&sdl_context, window, &gl);

    engine.start();
}
