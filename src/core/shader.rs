use std::collections::HashMap;

use glow::{HasContext, NativeProgram, NativeUniformLocation, FRAGMENT_SHADER, GEOMETRY_SHADER, VERTEX_SHADER};
use zerocopy::AsBytes;

use super::{matrix4f::Matrix4f, vector3f::Vector3f};

pub struct Shader {
    program: NativeProgram,
    uniforms: HashMap<String, NativeUniformLocation>
}

impl Shader {
    pub fn new(gl: &glow::Context) -> Shader {
        let program: NativeProgram;
        let uniforms: HashMap<String, NativeUniformLocation> = HashMap::new();
        unsafe {
            program = match gl.create_program() {
                Ok(program) => program,
                Err(err) => panic!("Program creation failed: {}", err),
            };
        }

        Shader {
            program,
            uniforms,
        }
    }

    pub fn add_vertex_shader(&self, text: String, gl: &glow::Context) {
        self.add_program(text, VERTEX_SHADER, gl);
    }

    pub fn add_geometry_shader(&self, text: String, gl: &glow::Context) {
        self.add_program(text, GEOMETRY_SHADER, gl);
    }

    pub fn add_fragment_shader(&self, text: String, gl: &glow::Context) {
        self.add_program(text, FRAGMENT_SHADER, gl);
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }

    pub fn add_uniform(&mut self, uniform: &str, gl: &glow::Context) {
        unsafe {
            let uniform_location = match gl.get_uniform_location(self.program, uniform) {
                    Some(location) => location,
                    None => panic!("Couldn't get uniform location: {}", uniform),
                };
            self.uniforms.insert(String::from(uniform), uniform_location);
        }
    }

    pub fn compile_shader(&self, gl: &glow::Context) {
        unsafe {
            gl.link_program(self.program);
            if !gl.get_program_link_status(self.program) {
                panic!("Failed to link program: {}", gl.get_program_info_log(self.program));
            }
        }
    }

    fn add_program(&self, text: String, shader_type: u32, gl: &glow::Context) {
        unsafe {
            let shader = match gl.create_shader(shader_type) {
                Ok(shader) => shader,
                Err(err) => panic!("Failed to create shader: {}", err),
            };

            gl.shader_source(shader, text.as_str());
            gl.compile_shader(shader);

            if !gl.get_shader_compile_status(shader) {
                panic!("Failed to compile shader: {}", gl.get_shader_info_log(shader));
            }

            gl.attach_shader(self.program, shader);
        }
    }
}

pub trait SetUniforms<T> {
    fn set_uniform(&self, uniform: &str, value: T, gl: &glow::Context);
}

impl SetUniforms<i32> for Shader {
    fn set_uniform(&self, uniform: &str, value: i32, gl: &glow::Context) {
        unsafe {
            gl.uniform_1_i32(self.uniforms.get(uniform), value);
        };
    }
}

impl SetUniforms<f32> for Shader {
    fn set_uniform(&self, uniform: &str, value: f32, gl: &glow::Context) {
        unsafe {
            gl.uniform_1_f32(self.uniforms.get(uniform), value);
        };
    }
}

impl SetUniforms<Vector3f> for Shader {
    fn set_uniform(&self, uniform: &str, value: Vector3f, gl: &glow::Context) {
        unsafe {
            gl.uniform_3_f32(self.uniforms.get(uniform), value.get_x(), value.get_y(), value.get_z());
        };
    }
}

impl SetUniforms<Matrix4f> for Shader {
    fn set_uniform(&self, uniform: &str, value: Matrix4f, gl: &glow::Context) {
        unsafe {
            gl.uniform_matrix_4_f32_slice(self.uniforms.get(uniform), true, &value.serialize())
        };
    }
}
