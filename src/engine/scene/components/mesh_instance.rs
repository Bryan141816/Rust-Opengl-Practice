use crate::engine::scene::resources::{MeshHandle, MaterialHandle};

pub struct MeshInstance { 
    mesh: MeshHandle,
    material: MaterialHandle
}

impl MeshInstance {
    pub fn new(mesh: MeshHandle, material: MaterialHandle)->Self {
        Self {
            mesh,
            material
        }
    }
}