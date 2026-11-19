use glam::Mat4;

use crate::engine::{
    lighting::Material,
    mesh::MeshHandle,
};

pub struct RenderObject {
    pub mesh: MeshHandle,
    pub material: Material,
    pub transform: Mat4,
}