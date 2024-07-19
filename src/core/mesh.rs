use glow::{HasContext, ARRAY_BUFFER, FLOAT, STATIC_DRAW, TRIANGLES};

use super::vertex::{self, Vertex, SIZE};
use super::utils;

pub struct Mesh<'a> {
    gl: &'a glow::Context,
    buffer: Option<glow::Buffer>,
    vao: Option<glow::VertexArray>,
    size: i32,
}

impl<'a> Mesh<'a> {
    pub fn new(gl: &'a glow::Context) -> Mesh<'a> {
        let buffer = unsafe { gl.create_buffer().ok() };
        let vao = unsafe { gl.create_vertex_array().ok() };

        Mesh {
            gl,
            buffer,
            vao,
            size: 0,
        }
    }

    pub fn add_vertices(&mut self, vertices: Vec<Vertex>) {
        self.size = vertices.len() as i32;

        let bytes = utils::vertices_to_bytes(vertices);

        unsafe{
            self.gl.bind_vertex_array(self.vao);
            self.gl.bind_buffer(ARRAY_BUFFER, self.buffer);
            self.gl.buffer_data_u8_slice(ARRAY_BUFFER, &bytes, STATIC_DRAW);
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.gl.enable_vertex_attrib_array(0);

            self.gl.bind_buffer(ARRAY_BUFFER, self.buffer);
            self.gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, vertex::SIZE * 4, 0);

            self.gl.draw_arrays(TRIANGLES, 0, self.size);

            self.gl.disable_vertex_attrib_array(0);
        }
    }
}
