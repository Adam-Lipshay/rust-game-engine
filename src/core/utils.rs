use super::vertex::Vertex;

pub fn vertices_to_byte_slice<'a>(vertices: Vec<Vertex>) -> &'a [u8] {
    let mut data: Vec<f32> = vec![];
    for vertex in vertices {
        data.extend(&vertex.get_pos().serialize())
    }

    unsafe {
        std::slice::from_raw_parts(data.as_ptr() as *const _, data.len() * 4)
    }
}
