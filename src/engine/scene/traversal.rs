use glam::Mat4;

use crate::engine::scene::{Node, NodeId, Scene};

pub struct SceneTraversal;

impl SceneTraversal {
    // Immutable traversal
    pub fn traverse<F>(
        scene: &Scene,
        root_id: NodeId,
        callback: &mut F,
    )
    where
        F: FnMut(&Node, Mat4, usize),
    {
        Self::traverse_node(scene, root_id, Mat4::IDENTITY, 0, callback);
    }

    fn traverse_node<F>(
        scene: &Scene,
        node_id: NodeId,
        parent_transform: Mat4,
        depth: usize,
        callback: &mut F,
    )
    where
        F: FnMut(&Node, Mat4, usize),
    {
        let Some(node) = scene.nodes.get(&node_id) else {
            return;
        };

        let world_transform = parent_transform * node.transform.to_matrix();

        callback(node, world_transform, depth);

        for child_id in &node.children {
            Self::traverse_node(
                scene,
                *child_id,
                world_transform,
                depth + 1,
                callback,
            );
        }
    }
}