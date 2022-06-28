use crate::system::rendering::mesh::vertex::Vertex;

pub struct Mesh{
    pub vertices: Box<[Vertex]>,
    pub indices: Box<[u32]>,
    pub faces: Box<[u16]>,
    // material:
}

