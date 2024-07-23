use super::vector3f::Vector3f;

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

        Camera { pos, forward, up }
    }
}
