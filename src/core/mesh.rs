use glow::{HasContext, ARRAY_BUFFER, FLOAT, STATIC_DRAW, TRIANGLES};

use super::vertex::{self, Vertex, SIZE};
use super::utils;

pub struct Mesh {
    buffer: Option<glow::Buffer>,
    size: i32,
}

impl Mesh {
    pub fn new(gl: &glow::Context) -> Mesh {
        let buffer: Option<glow::Buffer>;
        unsafe {
            buffer = gl.create_buffer().ok();
        }

        Mesh {
            buffer,
            size: 0,
        }
    }

    pub fn add_vertices(&mut self, gl: &glow::Context, vertices: Vec<Vertex>) {
        self.size = vertices.len() as i32;

        let bytes = utils::vertices_to_bytes(vertices);
        let bytes = dbg!(bytes);

        unsafe{
            gl.bind_buffer(ARRAY_BUFFER, self.buffer);
            gl.buffer_data_u8_slice(ARRAY_BUFFER, &bytes, STATIC_DRAW);
        }
    }

    pub fn draw(&self, gl: &glow::Context) {
        unsafe {
            gl.enable_vertex_attrib_array(0);

            gl.bind_buffer(ARRAY_BUFFER, self.buffer);
            gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, vertex::SIZE * 4, 0);

            gl.draw_arrays(TRIANGLES, 0, self.size);

            gl.disable_vertex_attrib_array(0);
        }
    }
}
