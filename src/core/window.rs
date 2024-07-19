use sdl2::{Sdl, video::{GLProfile, GLContext}};

use super::render_utils;

pub struct Window {
    video_subsystem: sdl2::VideoSubsystem,
    _gl_context: GLContext,
    window: sdl2::video::Window,
}

impl Window {
    pub fn new(sdl_context: &Sdl, width: u32, height: u32, title: &str, fullscreen: bool) -> (glow::Context, Window) {
        sdl2::hint::set("SDL_RENDER_VSYNC_DISPLAY", "0");
        sdl2::hint::set("vsync", "0");
        let video_subsystem = sdl_context.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        //gl_attr.set_context_version(3, 1);

        video_subsystem.gl_set_swap_interval(0);

        let mut window = video_subsystem.window(&title, width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        if fullscreen {
            window.set_fullscreen(sdl2::video::FullscreenType::True);
        }

        let _gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&_gl_context).unwrap();
        let gl: glow::Context;
        unsafe {
            gl = glow::Context::from_loader_function(|s| video_subsystem.gl_get_proc_address(s) as *const _);
        };

        let win = Window {
            video_subsystem,
            _gl_context,
            window,
        };

        (gl, win)
    }

    pub fn render(&self) {
        self.window.gl_swap_window();
    }

    pub fn get_width(&self) -> u32 {
        self.window.size().0
    }

    pub fn get_height(&self) -> u32 {
        self.window.size().1
    }

    pub fn get_title(&self) -> &str {
        self.window.title()
    }

    pub fn set_width(&mut self, width: u32) {
        self.window.set_size(width, self.get_height());
    }

    pub fn set_height(&mut self, height: u32) {
        self.window.set_size(self.get_height(), height);
    }
}
