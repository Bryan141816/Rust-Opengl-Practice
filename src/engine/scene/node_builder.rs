use crate::engine::scene::{Transform, component, transform};
use super::component::Component;
use super::scene::Scene;
use super::NodeId;


pub (crate) struct  NodeBuilder<'a> {
    pub(crate) scene: &'a mut Scene,
    pub(crate)  current_id: NodeId,
} 
impl<'a> NodeBuilder<'a> {
    pub fn child<F>(
        &mut self,
        name: impl Into<String>,
        build: F,
    ) -> &mut Self
    where
        F: FnOnce(&mut NodeBuilder),
    {

        let child_id = self.scene.insert_node(
            name,
            Some(self.current_id),
        );
        {
            let mut builder = NodeBuilder {
                scene: self.scene,
                current_id: child_id,
            };

            build(&mut builder);
        }
        self
    }
    pub fn id(&self) -> NodeId {
        self.current_id
    }
    pub fn component(&mut self, component: Component) -> &mut Self {
        self.scene.insert_component(self.current_id, component);
        self
    }
    pub fn transform(&mut self, transform: Transform) -> &mut Self {
        self.scene.update_transform(self.current_id, transform);
        self
    }
}