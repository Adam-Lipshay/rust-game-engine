use std::fs::read_to_string;

pub fn load_shader(filename: &str) -> String {

    let path = format!("./res/shaders/{}", filename);

    match read_to_string(path) {
        Ok(shader) => { shader }
        Err(err) => { panic!("Failed to load shader: {}",err) }
    }
}
