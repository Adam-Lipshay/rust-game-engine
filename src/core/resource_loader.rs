use std::fs::read_to_string;

use super::mesh::Mesh;
use super::vector3f::Vector3f;
use super::vertex::Vertex;

pub fn load_shader(file_name: &str) -> String {
    let path = format!("./res/shaders/{}", file_name);

    match read_to_string(path) {
        Ok(shader) => shader,
        Err(err) => {
            panic!("Failed to load shader: {}", err)
        }
    }
}

pub fn load_mesh<'a>(file_name: &str, gl: &'a glow::Context) -> Mesh<'a> {
    let path = format!("./res/models/{}", file_name);
    if file_name.split(".").last().unwrap() != "obj" {
        panic!(
            "Unsupported format: {}",
            file_name.split(".").last().unwrap()
        );
    }

    let obj = match read_to_string(path) {
        Ok(obj) => obj,
        Err(err) => {
            panic!("Failed to load obj: {}", err)
        }
    };

    let mut vertices: Vec<Vertex> = vec![];
    for vertice_line in obj.lines().filter(|c| c.chars().nth(0).unwrap() == 'v') {
        let tokens: Vec<&str> = vertice_line.split(" ").collect();
        vertices.push(Vertex::new(Vector3f::new(
            tokens[1].parse().unwrap(),
            tokens[2].parse().unwrap(),
            tokens[3].parse().unwrap(),
        )))
    }

    let mut indices: Vec<i32> = vec![];
    for index_line in obj.lines().filter(|c| c.chars().nth(0).unwrap() == 'f') {
        let tokens: Vec<&str> = index_line.split(" ").collect();
        indices.extend([
            tokens[1].parse::<i32>().unwrap() - 1,
            tokens[2].parse::<i32>().unwrap() - 1,
            tokens[3].parse::<i32>().unwrap() - 1,
        ]);
    }

    let mut mesh = Mesh::new(gl);
    mesh.add_vertices(vertices, indices);

    mesh
}
