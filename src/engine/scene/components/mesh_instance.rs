use crate::engine::scene::resources::{MeshHandle, MaterialHandle};
#[derive(Debug)]
pub struct MeshInstance { 
    pub mesh: MeshHandle,
    pub material: MaterialHandle
}

impl MeshInstance {
    pub fn new(mesh: MeshHandle, material: MaterialHandle)->Self {
        Self {
            mesh,
            material
        }
    }
}