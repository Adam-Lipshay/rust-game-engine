use crate::core;
use sdl2::{keyboard::Scancode, mouse::MouseButton};

use super::{
    camera::Camera,
    mesh, resource_loader,
    shader::{self, SetUniforms},
    time,
    transform::{self, TransformationSetters},
    vector3f,
};

pub struct Game<'a> {
    gl: &'a glow::Context,
    temp: f32,
    transform: transform::Transform,
    mesh: mesh::Mesh<'a>,
    shader: shader::Shader<'a>,
}

impl<'a> Game<'a> {
    pub fn new(gl: &'a glow::Context) -> Game<'a> {
        // let mut mesh = core::mesh::Mesh::new(gl);

        // let vertices: Vec<Vertex> = vec![Vertex::new(Vector3f::new(-1.0, -1.0, 0.0)),
        //                                  Vertex::new(Vector3f::new(0.0, 1.0, 0.0)),
        //                                  Vertex::new(Vector3f::new(1.0, -1.0, 0.0)),
        //                                  Vertex::new(Vector3f::new(0.0, -1.0, 1.0))];

        // let indices: Vec<i32> = vec![0, 1, 3,
        //                              3, 1, 2,
        //                              2, 1, 0,
        //                              0, 2, 3];

        // mesh.add_vertices(vertices, indices);
        let mesh = resource_loader::load_mesh("box.obj", gl).unwrap();

        let mut shader = shader::Shader::new(gl);

        shader.add_vertex_shader(resource_loader::load_shader("basic_vertex.glsl"));
        shader.add_fragment_shader(resource_loader::load_shader("basic_fragment.glsl"));
        shader.compile_shader();

        shader.add_uniform("transform");

        let cam = Camera::new(
            vector3f::Vector3f::new(0.0, 0.0, 0.0),
            vector3f::Vector3f::new(0.0, 0.0, 1.0),
            vector3f::Vector3f::new(0.0, 1.0, 0.0),
        );

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
            println!(
                "Released left click at {}",
                input.get_mouse_position().to_string()
            );
        }
    }

    pub fn update(&mut self, time: &time::Time) {
        self.temp += time.get_delta() as f32;
        let temp_sin = self.temp.sin();
        self.transform.set_translation((0.0, 0.0, 5.0));
        self.transform.set_rotation((0.0, temp_sin * 180.0, 0.0));
        //self.transform
        //    .set_scale((0.5,0.5,0.5));
    }

    pub fn render(&mut self) {
        self.shader.bind();
        self.shader
            .set_uniform("transform", self.transform.get_projection_transformation());
        self.mesh.draw();
    }
}
