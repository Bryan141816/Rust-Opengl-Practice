use glam::Mat4;
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModelNode {
    pub name: Option<String>,
    pub transform: Mat4,
    pub meshes: Vec<usize>,
    pub children: Vec<ModelNode>,
}

impl ModelNode {
    pub fn new(
        name: Option<String>,
        transform: Mat4,
        meshes: Vec<usize>,
        children: Vec<ModelNode>,
    ) -> Self {
        Self {
            name,
            transform,
            meshes,
            children,
        }
    }
}