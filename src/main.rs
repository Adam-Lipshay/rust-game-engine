use sdl2;

#[allow(dead_code)]
mod core;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut engine = core::engine::Engine::new(&sdl_context);

    engine.start();
}
