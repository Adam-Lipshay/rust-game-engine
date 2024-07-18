use std::{thread, time::Duration};
use crate::core;

use super::render_utils;

const FRAME_CAP: f64 = 1000.0;

pub struct Engine {
    is_running: bool,
    window: core::window::Window,
    gl: glow::Context,
    events: core::events::EventHandler,
    time: core::time::Time,
    game: core::game::Game,
    input: core::input::Input,
}

impl Engine {
    pub fn new(sdl_context: &sdl2::Sdl) -> Engine {
        let mut event_handler = core::events::EventHandler::new(&sdl_context);
        let (gl, window) = core::window::Window::new(&sdl_context, 800, 600, "test", false);

        render_utils::init_graphics(&gl);

        Engine {
            is_running: false,
            game: core::game::Game::new(&gl),
            window,
            gl,
            input: core::input::Input::new(event_handler.get_event_pump()),
            events: event_handler,
            time: core::time::Time::new(),
        }
    }

    pub fn start(&mut self) {
        if self.is_running {
            return;
        }


        self.run();
    }

    pub fn stop(&mut self) {
        if !self.is_running {
            return;
        }

        self.is_running = false;
    }

    fn run(&mut self) {
        self.is_running = true;

        let mut frames = 0;
        let mut frames_counter = 0;

        const FRAME_TIME: f64 = 1.0 / FRAME_CAP;

        let mut last_time = self.time.get_time();
        let mut unprocessed_time: f64 = 0.0;

        while self.is_running {
            let mut do_render = false;

            let start_time = self.time.get_time();
            let passed_time = start_time - last_time;
            last_time = start_time;

            unprocessed_time += passed_time as f64 / core::time::SECOND as f64;
            frames_counter += passed_time;

            while unprocessed_time > FRAME_TIME {
                do_render = true;

                unprocessed_time -= FRAME_TIME;

                self.input.update(self.events.get_event_pump());
                self.events.poll_events(&mut self.input);
                if self.events.get_is_close_requested() {
                    self.stop();
                }

                self.time.set_delta(FRAME_TIME);

                self.game.input(&mut self.input);
                self.game.update();

                if frames_counter >= core::time::SECOND {
                    println!("{}", frames);
                    frames = 0;
                    frames_counter = 0;
                }
            }

            if do_render {
                self.render();
                frames += 1;
            } else {
                thread::sleep(Duration::from_millis(1));
            }
        }
    }

    fn render(&mut self) {
        render_utils::clear_screen(&self.gl);
        self.game.render(&self.gl);
        self.window.render();
    }
}
