use sdl2::{EventPump, Sdl, event::Event};
use crate::core;

pub struct EventHandler {
    event_pump: EventPump,
    is_close_requested: bool,
}

impl EventHandler {
    pub fn new(sdl_context: &Sdl) -> EventHandler {
        EventHandler {
            event_pump: sdl_context.event_pump().unwrap(),
            is_close_requested: false,
        }
    }

    pub fn poll_events(&mut self, input: &mut core::input::Input) {
        for event in self.event_pump.poll_iter(){
            match event {
                Event::Quit {..} => {
                    self.is_close_requested = true;
                },
                Event::KeyDown { scancode, .. } => {
                    input.set_down_key(scancode.unwrap(), true);
                },
                Event::KeyUp { scancode, .. } => {
                    input.set_down_key(scancode.unwrap(), false);
                    input.set_up_key(scancode.unwrap(), true);
                },
                Event::MouseButtonDown { mouse_btn, ..} => {
                    input.set_mouse_button_down(mouse_btn, true);
                },
                Event::MouseButtonUp { mouse_btn, ..} => {
                    input.set_mouse_button_down(mouse_btn, false);
                    input.set_mouse_button_up(mouse_btn, true);
                },
                _ => {}
            }
        }
    }

    pub fn get_is_close_requested(&mut self) -> bool {
        self.is_close_requested
    }

    pub fn get_event_pump(&mut self) -> &mut EventPump {
        &mut self.event_pump
    }
}
