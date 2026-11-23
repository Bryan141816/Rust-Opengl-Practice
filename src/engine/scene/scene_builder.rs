use glfw::Key::P;

use crate::engine::scene::Scene;
use crate::engine::scene::node_builder::NodeBuilder;
use crate::engine::scene::{Node, NodeId, NodeName, Transform};
use super::component::Component;
pub (crate) struct SceneBuilder {
    pub(crate) scene: Scene,
}
impl SceneBuilder {
    pub fn node<F>(
        mut self,
        build: F,
    ) -> Self
    where 
        F: FnOnce(&mut NodeBuilder),
    {
        let id = self.scene.insert_node("Root", None, );
        {
            let mut builder = NodeBuilder {
                scene: &mut self.scene,
                current_id: id,
            };

            build(&mut builder);
        }
        self
    }
    pub fn build(self) -> Scene {
        self.scene
    }
}