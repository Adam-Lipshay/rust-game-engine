use glow::{HasContext, NativeProgram, FRAGMENT_SHADER, GEOMETRY_SHADER, VERTEX_SHADER};

pub struct Shader {
    program: NativeProgram,
}

impl Shader {
    pub fn new(gl: &glow::Context) -> Shader {
        let program: NativeProgram;
        unsafe {
            program = match gl.create_program() {
                Ok(program) => program,
                Err(err) => panic!("Program creation failed: {}", err),
            };
        }

        Shader {
            program,
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

    pub fn bind_progam(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.program));
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
