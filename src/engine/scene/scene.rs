use crate::engine::scene::Node;
#[derive(Debug)]
pub struct Scene {
    pub root: Node,
    pub active_camera: String
}

impl Scene {
    pub fn new(active_camera: impl Into<String>) -> Self {
        Self {
            root: Node::new("Root"),
            active_camera: active_camera.into()
        }   
    }
    fn set_active_camera(&mut self, active_camera: String) {
        self.active_camera = active_camera;
    }
}