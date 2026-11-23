
use crate::engine::scene::{Node, NodeId, Transform, scene_builder::SceneBuilder, transform};
use indexmap::IndexMap;
use super::component::Component;
#[derive(Debug)]
pub struct Scene {
    pub nodes: IndexMap<NodeId, Node>,
    pub active_camera: String,
    next_id: NodeId
}

impl Scene {
    pub fn new(active_camera: impl Into<String>) -> SceneBuilder {
        SceneBuilder {
            scene: Self {
                nodes: IndexMap::new(),
                active_camera: active_camera.into(),
                next_id: 0,
            },
        }
    }
    pub fn insert_node(
        &mut self,
        name: impl Into<String>,
        parent: Option<NodeId>,
    ) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;

        let node = Node {
            id,
            name: name.into(),
            parent,
            transform: Transform::new(),
            components: Vec::new(),
            children: Vec::new(),
        };
        self.nodes.insert(id, node);
        // Register this node in its parent's children
        if let Some(parent_id) = parent {
            if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
                parent_node.children.push(id);
            }
        }

        id
    }
    pub fn insert_component(&mut self, id: NodeId, component: Component) -> &mut Self
    {
        if let Some(parent_node) = self.nodes.get_mut(&id) {
            parent_node.components.push(component);
        }
        self
    }
    pub fn update_transform(&mut self, id: NodeId, transform: Transform) -> &mut Self {
        if let Some(parent_node) = self.nodes.get_mut(&id) {
            parent_node.transform = transform;
        }
        self
    }
}
//Get Node Impl
impl Scene {
    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }
}
//Find Node Impl
impl Scene {
    pub fn find_node(&self, name: &str) -> Option<&Node> {
        self.nodes.values().find(|node| node.name == name)
    }

    pub fn find_node_mut(&mut self, name: &str) -> Option<&mut Node> {
        self.nodes.values_mut().find(|node| node.name == name)
    }
}
//Get Children
impl Scene {
    pub fn get_children(&self, parent_id: NodeId) -> Vec<&Node> {
        self.nodes
            .get(&parent_id)
            .map(|parent| {
                parent.children
                    .iter()
                    .filter_map(|child_id| self.nodes.get(child_id))
                    .collect()
            })
            .unwrap_or_default()
    }
}