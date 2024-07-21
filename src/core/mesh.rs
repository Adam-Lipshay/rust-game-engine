use glow::{
    HasContext, ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER, FLOAT, STATIC_DRAW, TRIANGLES, UNSIGNED_INT,
};

use super::utils;
use super::vertex::{self, Vertex};

pub struct Mesh<'a> {
    gl: &'a glow::Context,
    vbo: Option<glow::Buffer>,
    ibo: Option<glow::Buffer>,
    vao: Option<glow::VertexArray>,
    size: i32,
}

impl<'a> Mesh<'a> {
    pub fn new(gl: &'a glow::Context) -> Mesh<'a> {
        let vbo = unsafe { gl.create_buffer().ok() };
        let ibo = unsafe { gl.create_buffer().ok() };
        let vao = unsafe { gl.create_vertex_array().ok() };

        Mesh {
            gl,
            vbo,
            ibo,
            vao,
            size: 0,
        }
    }

    pub fn add_vertices(&mut self, vertices: Vec<Vertex>, indices: Vec<i32>) {
        self.size = indices.len() as i32;

        unsafe {
            self.gl.bind_vertex_array(self.vao);
            self.gl.bind_buffer(ARRAY_BUFFER, self.vbo);
            self.gl.buffer_data_u8_slice(
                ARRAY_BUFFER,
                &utils::vertices_to_bytes(vertices),
                STATIC_DRAW,
            );

            self.gl.bind_buffer(ELEMENT_ARRAY_BUFFER, self.ibo);
            self.gl.buffer_data_u8_slice(
                ELEMENT_ARRAY_BUFFER,
                &utils::indices_to_bytes(indices),
                STATIC_DRAW,
            );
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.gl.enable_vertex_attrib_array(0);

            self.gl.bind_buffer(ARRAY_BUFFER, self.vbo);
            self.gl
                .vertex_attrib_pointer_f32(0, 3, FLOAT, false, vertex::SIZE * 4, 0);

            self.gl.bind_buffer(ELEMENT_ARRAY_BUFFER, self.ibo);
            self.gl.draw_elements(TRIANGLES, self.size, UNSIGNED_INT, 0);

            self.gl.disable_vertex_attrib_array(0);
        }
    }
}
