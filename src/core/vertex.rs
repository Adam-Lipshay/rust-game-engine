use super::vector3f::Vector3f;

pub const SIZE: i32 = 3;

#[derive(zerocopy::AsBytes, Debug)]
#[repr(C)]
pub struct Vertex {
    pos: Vector3f,
}

impl Vertex {
    pub fn new(pos: Vector3f) -> Vertex {
        Vertex { pos }
    }

    pub fn get_pos(&self) -> Vector3f {
        self.pos
    }

    pub fn set_pos(&mut self, pos: Vector3f) {
        self.pos = pos;
    }
}
