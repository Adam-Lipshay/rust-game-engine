use std::collections::HashMap;
use glow::{HasContext, NativeProgram, NativeUniformLocation, FRAGMENT_SHADER, GEOMETRY_SHADER, VERTEX_SHADER};
use super::{matrix4f::Matrix4f, vector3f::Vector3f};

pub struct Shader<'a> {
    gl: &'a glow::Context,
    program: NativeProgram,
    uniforms: HashMap<String, NativeUniformLocation>
}

impl<'a> Shader<'a> {
    pub fn new(gl: &'a glow::Context) -> Shader<'a> {
        let program: NativeProgram;
        let uniforms: HashMap<String, NativeUniformLocation> = HashMap::new();
        unsafe {
            program = match gl.create_program() {
                Ok(program) => program,
                Err(err) => panic!("Program creation failed: {}", err),
            };
        }

        Shader {
            gl,
            program,
            uniforms,
        }
    }

    pub fn add_vertex_shader(&self, text: String) {
        self.add_program(text, VERTEX_SHADER);
    }

    pub fn add_geometry_shader(&self, text: String) {
        self.add_program(text, GEOMETRY_SHADER);
    }

    pub fn add_fragment_shader(&self, text: String) {
        self.add_program(text, FRAGMENT_SHADER);
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.use_program(Some(self.program));
        }
    }

    pub fn add_uniform(&mut self, uniform: &str) {
        unsafe {
            let uniform_location = match self.gl.get_uniform_location(self.program, uniform) {
                    Some(location) => location,
                    None => panic!("Couldn't get uniform location: {}", uniform),
                };
            self.uniforms.insert(String::from(uniform), uniform_location);
        }
    }

    pub fn compile_shader(&self) {
        unsafe {
            self.gl.link_program(self.program);
            if !self.gl.get_program_link_status(self.program) {
                panic!("Failed to link program: {}", self.gl.get_program_info_log(self.program));
            }
        }
    }

    fn add_program(&self, text: String, shader_type: u32) {
        unsafe {
            let shader = match self.gl.create_shader(shader_type) {
                Ok(shader) => shader,
                Err(err) => panic!("Failed to create shader: {}", err),
            };

            self.gl.shader_source(shader, text.as_str());
            self.gl.compile_shader(shader);

            if !self.gl.get_shader_compile_status(shader) {
                panic!("Failed to compile shader: {}", self.gl.get_shader_info_log(shader));
            }

            self.gl.attach_shader(self.program, shader);
        }
    }
}

pub trait SetUniforms<T> {
    fn set_uniform(&self, uniform: &str, value: T);
}

impl<'a> SetUniforms<i32> for Shader<'a> {
    fn set_uniform(&self, uniform: &str, value: i32) {
        unsafe {
            self.gl.uniform_1_i32(self.uniforms.get(uniform), value);
        };
    }
}

impl<'a> SetUniforms<f32> for Shader<'a> {
    fn set_uniform(&self, uniform: &str, value: f32) {
        unsafe {
            self.gl.uniform_1_f32(self.uniforms.get(uniform), value);
        };
    }
}

impl<'a> SetUniforms<Vector3f> for Shader<'a> {
    fn set_uniform(&self, uniform: &str, value: Vector3f) {
        unsafe {
            self.gl.uniform_3_f32(self.uniforms.get(uniform), value.get_x(), value.get_y(), value.get_z());
        };
    }
}

impl<'a> SetUniforms<Matrix4f> for Shader<'a> {
    fn set_uniform(&self, uniform: &str, value: Matrix4f) {
        unsafe {
            self.gl.uniform_matrix_4_f32_slice(self.uniforms.get(uniform), true, &value.serialize())
        };
    }
}
