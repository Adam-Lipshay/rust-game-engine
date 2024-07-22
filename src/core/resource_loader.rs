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

pub fn load_mesh<'a>(file_name: &str, gl: &'a glow::Context) -> Result<Mesh<'a>, String> {
    let path = format!("./res/models/{}", file_name);
    if file_name.split('.').last() != Some("obj") {
        return Err(format!(
            "Unsupported format: {}",
            file_name.split('.').last().unwrap_or("")
        ));
    }

    let obj = read_to_string(&path).map_err(|err| format!("Failed to load obj: {}", err))?;

    let mut vertices: Vec<Vertex> = vec![];
    for vertice_line in obj.lines().filter(|c| c.starts_with('v')) {
        let tokens: Vec<&str> = vertice_line.split_whitespace().collect();
        vertices.push(Vertex::new(Vector3f::new(
            tokens[1].parse().map_err(|_| "Failed to parse vertex x")?,
            tokens[2].parse().map_err(|_| "Failed to parse vertex y")?,
            tokens[3].parse().map_err(|_| "Failed to parse vertex z")?,
        )));
    }

    let mut indices: Vec<i32> = vec![];
    for index_line in obj.lines().filter(|c| c.starts_with('f')) {
        let tokens: Vec<&str> = index_line.split_whitespace().collect();
        indices.extend([
            tokens[1].parse::<i32>().map_err(|_| "Failed to parse index 1")? - 1,
            tokens[2].parse::<i32>().map_err(|_| "Failed to parse index 2")? - 1,
            tokens[3].parse::<i32>().map_err(|_| "Failed to parse index 3")? - 1,
        ]);
    }

    let mut mesh = Mesh::new(gl);
    mesh.add_vertices(vertices, indices);

    Ok(mesh)
}
