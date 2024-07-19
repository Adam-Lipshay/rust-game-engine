use glow::{HasContext, COLOR_BUFFER_BIT, CULL_FACE, DEPTH_BUFFER_BIT, DEPTH_TEST, FRAMEBUFFER_SRGB, VERSION};

pub fn clear_screen(gl: &glow::Context) {
    unsafe {
        gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
    }
}

pub fn init_graphics(gl: &glow::Context) {
    unsafe {
        gl.clear_color(0.0, 0.0, 0.0, 0.0);

        gl.front_face(glow::CW);
        gl.cull_face(glow::BACK);
        gl.enable(CULL_FACE);
        gl.enable(DEPTH_TEST);

        gl.enable(FRAMEBUFFER_SRGB);
    }
}

pub fn get_opengl_version(gl: &glow::Context) -> String {
    unsafe {
        gl.get_parameter_string(VERSION)
    }
}
