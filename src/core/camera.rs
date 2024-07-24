use super::{
    input,
    vector3f::{Math, Vector3f},
};
use sdl2::keyboard::Scancode;

static mut Y_AXIS: Option<Vector3f> = None;

pub struct Camera {
    pos: Vector3f,
    forward: Vector3f,
    up: Vector3f,
}

impl Camera {
    pub fn new(pos: Vector3f, forward: Vector3f, up: Vector3f) -> Camera {
        unsafe {
            Y_AXIS = Some(Vector3f::new(0.0, 1.0, 0.0));
        }

        let mut forward = forward;
        let mut up = up;

        forward.normalize();
        up.normalize();

        Camera { pos, forward, up }
    }

    pub fn input(&mut self, input: &mut input::Input) {
        let mov_amt = 10.0 * 0.001;
        let rot_amt = 100.0 * 0.001;

        if input.is_key_down(Scancode::W) {
            self.move_camera(self.forward, mov_amt);
        }

        if input.is_key_down(Scancode::S) {
            self.move_camera(self.forward, -mov_amt);
        }

        if input.is_key_down(Scancode::A) {
            self.move_camera(self.get_left(), mov_amt);
        }

        if input.is_key_down(Scancode::D) {
            self.move_camera(self.get_right(), mov_amt);
        }

        if input.is_key_down(Scancode::Up) {
            self.rotate_x(-rot_amt);
        }

        if input.is_key_down(Scancode::Down) {
            self.rotate_x(rot_amt);
        }

        if input.is_key_down(Scancode::Left) {
            self.rotate_y(-rot_amt);
        }

        if input.is_key_down(Scancode::Right) {
            self.rotate_y(rot_amt);
        }
    }

    pub fn move_camera(&mut self, dir: Vector3f, amt: f32) {
        self.pos = self.pos.add(dir.mul(amt));
    }

    pub fn get_left(&self) -> Vector3f {
        let mut left = self.forward.cross(self.up);
        left.normalize();

        left
    }

    pub fn get_right(&self) -> Vector3f {
        let mut right = self.up.cross(self.forward);
        right.normalize();

        right
    }

    pub fn rotate_y(&mut self, angle: f32) {
        let mut h_axis;
        unsafe {
            h_axis = Y_AXIS.unwrap().cross(self.forward);
            h_axis.normalize();

            self.forward.rotate(angle, Y_AXIS.unwrap());
        }

        self.forward.normalize();

        self.up = self.forward.cross(h_axis);
        self.up.normalize();
    }

    pub fn rotate_x(&mut self, angle: f32) {
        let mut h_axis;
        unsafe {
            h_axis = Y_AXIS.unwrap().cross(self.forward);
            h_axis.normalize();
        }

        self.forward.rotate(angle, h_axis);
        self.forward.normalize();

        self.up = self.forward.cross(h_axis);
        self.up.normalize();
    }

    pub fn get_pos(&self) -> Vector3f {
        self.pos
    }

    pub fn get_forward(&self) -> Vector3f {
        self.forward
    }

    pub fn get_up(&self) -> Vector3f {
        self.up
    }

    pub fn set_pos(&mut self, pos: Vector3f) {
        self.pos = pos;
    }

    pub fn set_forward(&mut self, forward: Vector3f) {
        self.forward = forward;
    }

    pub fn set_up(&mut self, up: Vector3f) {
        self.up = up;
    }
}

impl Default for Camera {
    fn default() -> Self {
        unsafe {
            Y_AXIS = Some(Vector3f::new(0.0, 1.0, 0.0));
        }

        Camera {
            pos: Vector3f::new(0.0, 0.0, 0.0),
            forward: Vector3f::new(0.0, 0.0, 1.0),
            up: Vector3f::new(0.0, 1.0, 0.0),
        }
    }
}
