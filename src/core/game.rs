use crate::core;
use sdl2::{keyboard::Scancode, mouse::MouseButton};

use super::{mesh, resource_loader, shader::{self, SetUniforms}, time, transform::{self, transformation_setters}, vector3f::Vector3f, vertex::Vertex};

pub struct Game<'a> {
    gl: &'a glow::Context,
    temp: f32,
    transform: transform::Transform,
    mesh: mesh::Mesh<'a>,
    shader: shader::Shader<'a>,
}

impl<'a> Game<'a> {
    pub fn new(gl: &'a glow::Context) -> Game<'a> {
        let mut mesh = core::mesh::Mesh::new(gl);

        let data: Vec<Vertex> = vec![Vertex::new(Vector3f::new(-1.0, -1.0, 0.0)),
                                     Vertex::new(Vector3f::new(0.0, 1.0, 0.0)),
                                     Vertex::new(Vector3f::new(1.0, -1.0, 0.0))];

        mesh.add_vertices(data);

        let mut shader = shader::Shader::new(gl);

        shader.add_vertex_shader(resource_loader::load_shader("basic_vertex.glsl"));
        shader.add_fragment_shader(resource_loader::load_shader("basic_fragment.glsl"));
        shader.compile_shader();

        shader.add_uniform("transform");
        Game {
            gl,
            temp: 0.0,
            transform: transform::Transform::new(),
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

    pub fn update(&mut self, time: &time::Time) {
        self.temp += time.get_delta() as f32;
        let temp_sin = self.temp.sin();
        self.transform.set_translation((temp_sin,0.0,0.0));
        self.transform.set_rotation((0.0,0.0,temp_sin * 180.0));
        self.transform.set_scale((temp_sin,temp_sin,temp_sin));
    }

    pub fn render(&mut self) {
        self.shader.bind();
        self.shader.set_uniform("transform", self.transform.get_transformation());
        self.mesh.draw();
    }
}
