use super::vertex::Vertex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshHandle(pub u32);

impl MeshHandle {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>
}
impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
        }
    }
}