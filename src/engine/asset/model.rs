use crate::engine::scene::resources::Mesh;

use super::model_node::ModelNode;

#[derive(Debug, Clone)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub roots: Vec<ModelNode>,
}

impl Model {
    pub fn new(meshes: Vec<Mesh>, roots: Vec<ModelNode>) -> Self {
        Self {
            meshes,
            roots,
        }
    }
}