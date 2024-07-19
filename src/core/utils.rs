use zerocopy::AsBytes;

use super::vertex::Vertex;

pub fn vertices_to_bytes(vertices: Vec<Vertex>) -> Vec<u8> {
    let mut data: Vec<u8> = vec![];
    for vertex in vertices {
        data.extend(vertex.as_bytes());
    }

    // let mut data_u8: Vec<u8> = vec![];
    // for float in data {
    //     data_u8.extend(float.to_le_bytes());
    // }
    data
}
