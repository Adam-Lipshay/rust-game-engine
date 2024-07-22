use sdl2;

#[allow(dead_code)]
mod core;

const INIT_WIDTH: u32 = 800;
const INIT_HEIGHT: u32 = 600;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let gl: glow::Context;
    let window: core::window::Window;
    (gl, window) = core::window::Window::new(&sdl_context, 800, 600, "test", false);
    let mut engine = core::engine::Engine::new(&sdl_context, window, &gl);

    engine.start();
}
