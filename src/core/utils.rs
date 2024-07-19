use super::vertex::Vertex;

pub fn vertices_to_bytes(vertices: Vec<Vertex>) -> Vec<u8> {
    let mut data: Vec<f32> = vec![];
    for vertex in vertices {
        data.extend(&vertex.get_pos().serialize())
    }

    let mut data_u8: Vec<u8> = vec![];
    for float in data {
        data_u8.extend(float.to_le_bytes());
    }

    data_u8
}
