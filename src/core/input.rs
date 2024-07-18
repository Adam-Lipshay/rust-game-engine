use sdl2::{keyboard::Scancode, mouse::{MouseButton, MouseState}, EventPump};

use super::vector2f::Vector2f;

pub struct Input {
    prior_keys: [bool; 256],
    down_keys: [bool; 256],
    up_keys: [bool; 256],
    mouse_state: MouseState,
    prior_mouse_buttons: [bool; 6],
    mouse_buttons_down: [bool; 6],
    mouse_buttons_up: [bool; 6],
}

impl Input {
    pub fn new(event_pump: &mut EventPump) -> Input {
        Input {
            prior_keys: [false; 256],
            down_keys: [false; 256],
            up_keys: [false; 256],
            mouse_state: MouseState::new(event_pump),
            prior_mouse_buttons: [false; 6],
            mouse_buttons_down: [false; 6],
            mouse_buttons_up: [false; 6],
        }
    }

    pub fn update(&mut self, event_pump: &mut EventPump) {
        self.prior_keys = self.down_keys;
        self.prior_mouse_buttons = self.mouse_buttons_down;
        self.up_keys = [false; 256];
        self.mouse_buttons_up = [false; 6];

        self.mouse_state = event_pump.mouse_state();
    }

    pub fn set_down_key(&mut self, scancode: Scancode, value: bool) {
        self.down_keys[scancode as usize] = value;
    }

    pub fn set_up_key(&mut self, scancode: Scancode, value: bool) {
        self.up_keys[scancode as usize] = value;
    }

    pub fn is_key_down(&mut self, scancode: Scancode) -> bool {
        self.down_keys[scancode as usize]
    }

    pub fn get_key_down(&mut self, scancode: Scancode) -> bool {
        self.down_keys[scancode as usize] && !self.prior_keys[scancode as usize]
    }

    pub fn get_key_up(&mut self, scancode: Scancode) -> bool {
        self.up_keys[scancode as usize]
    }

    pub fn set_mouse_button_down(&mut self, mouse_btn: MouseButton, value: bool) {
        self.mouse_buttons_down[mouse_btn as usize] = value;
    }

    pub fn set_mouse_button_up(&mut self, mouse_btn: MouseButton, value: bool) {
        self.mouse_buttons_up[mouse_btn as usize] = value;
    }

    pub fn get_mouse_button_down(&mut self, mouse_btn: MouseButton) -> bool {
        self.mouse_buttons_down[mouse_btn as usize] && !self.prior_mouse_buttons[mouse_btn as usize]
    }

    pub fn get_mouse_button_up(&mut self, mouse_btn: MouseButton) -> bool {
        self.mouse_buttons_up[mouse_btn as usize]
    }

    pub fn is_mouse_button_down(&mut self, mouse_btn: MouseButton) -> bool {
        self.mouse_buttons_down[mouse_btn as usize]
    }

    pub fn get_mouse_position(&self) -> Vector2f {
        Vector2f::new(self.mouse_state.x() as f32, self.mouse_state.y() as f32)
    }
}
