use crate::core;
use sdl2::{keyboard::Scancode, mouse::MouseButton};

use super::{mesh, resource_loader, shader, vector3f::Vector3f, vertex::Vertex};

pub struct Game {
    mesh: mesh::Mesh,
    shader: shader::Shader,
}

impl Game {
    pub fn new(gl: &glow::Context) -> Game {
        let mut mesh = core::mesh::Mesh::new(gl);

        let data: Vec<Vertex> = vec![Vertex::new(Vector3f::new(-1.0, -1.0, 0.0)),
                                     Vertex::new(Vector3f::new(0.0, 1.0, 0.0)),
                                     Vertex::new(Vector3f::new(1.0, -1.0, 0.0))];

        mesh.add_vertices(gl, data);

        let shader = shader::Shader::new(gl);

        shader.add_vertex_shader(resource_loader::load_shader("basic_vertex.glsl"), gl);
        shader.add_fragment_shader(resource_loader::load_shader("basic_fragment.glsl"), gl);
        shader.compile_shader(gl);
        Game {
            mesh,
            shader,
        }
    }

    pub fn input(&mut self, input: &mut core::input::Input) {
        if input.get_key_down(Scancode::W) {
            println!("W pressed down");
        }

        if input.get_key_up(Scancode::W) {
            println!("W released");
        }

        if input.get_mouse_button_down(MouseButton::Left) {
            println!("Left clicked at {}", input.get_mouse_position().to_string());
        }

        if input.get_mouse_button_up(MouseButton::Left) {
            println!("Released left click at {}", input.get_mouse_position().to_string());
        }
    }

    pub fn update(&mut self) {

    }

    pub fn render(&mut self, gl: &glow::Context) {
        self.shader.bind_progam(gl);
        self.mesh.draw(gl);
    }
}
