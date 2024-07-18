use sdl2;
mod core;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut engine = core::engine::Engine::new(&sdl_context);

    engine.start();
}
