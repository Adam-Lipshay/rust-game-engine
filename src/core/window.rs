use glow::HasContext;
use sdl2::{
    video::{GLContext, GLProfile},
    Sdl,
};

use super::transform;

pub struct Window {
    video_subsystem: sdl2::VideoSubsystem,
    _gl_context: GLContext,
    window: sdl2::video::Window,
}

impl Window {
    pub fn new(
        sdl_context: &Sdl,
        width: u32,
        height: u32,
        title: &str,
        fullscreen: bool,
    ) -> (glow::Context, Window) {
        let video_subsystem = sdl_context.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        //gl_attr.set_context_version(3, 1);

        //video_subsystem.gl_set_swap_interval(0);

        let mut window = video_subsystem
            .window(&title, width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        if fullscreen {
            match window.set_fullscreen(sdl2::video::FullscreenType::True) {
                Ok(_) => {}
                Err(err) => {
                    panic!("Failed to set fullscreen: {}", err)
                }
            };
        }

        let _gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&_gl_context).unwrap();
        let gl: glow::Context;
        unsafe {
            gl = glow::Context::from_loader_function(|s| {
                video_subsystem.gl_get_proc_address(s) as *const _
            });
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

    pub fn set_width(&mut self, width: u32, gl: &glow::Context) {
        match self.window.set_size(width, self.get_height()) {
            Ok(_) => {}
            Err(err) => panic!("Failed to change height: {}", err),
        };

        unsafe {
            gl.viewport(0, 0, width as i32, self.get_height() as i32);
            transform::set_projection(
                transform::get_fov(),
                width as f32,
                transform::get_height(),
                transform::get_z_near(),
                transform::get_z_far(),
            );
        }
    }

    pub fn set_height(&mut self, height: u32, gl: &glow::Context) {
        match self.window.set_size(self.get_width(), height) {
            Ok(_) => {}
            Err(err) => panic!("Failed to change height: {}", err),
        };

        unsafe {
            gl.viewport(0, 0, self.get_width() as i32, height as i32);
            transform::set_projection(
                transform::get_fov(),
                transform::get_width(),
                height as f32,
                transform::get_z_near(),
                transform::get_z_far(),
            );
        }
    }

    pub fn set_fullscreen(&mut self) {
        match self
            .window
            .set_fullscreen(sdl2::video::FullscreenType::True)
        {
            Ok(_) => {}
            Err(err) => {
                panic!("Failed to set fullscreen: {}", err)
            }
        };
    }
}
