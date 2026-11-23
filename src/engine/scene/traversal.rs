use glam::Mat4;

use crate::engine::scene::Node;

pub struct SceneTraversal;

impl SceneTraversal {
    pub fn traverse<F>(root: &Node, callback: &mut F)
    where
        F: FnMut(&Node, Mat4, usize),
    {
        Self::traverse_node(root, Mat4::IDENTITY, 0, callback);
    }

    fn traverse_node<F>(
        node: &Node,
        parent_transform: Mat4,
        depth: usize,
        callback: &mut F,
    )
    where
        F: FnMut(&Node, Mat4, usize),
    {
        let world_transform = parent_transform * node.transform.to_matrix();

        callback(node, world_transform, depth);

        for child in &node.children {
            Self::traverse_node(child, world_transform, depth + 1, callback);
        }
    }
}
impl SceneTraversal {
    pub fn traverse_mut<F>(root: &mut Node, callback: &mut F)
    where
        F: FnMut(&mut Node, Mat4, usize),
    {
        Self::traverse_node_mut(root, Mat4::IDENTITY, 0, callback);
    }

    fn traverse_node_mut<F>(
        node: &mut Node,
        parent_transform: Mat4,
        depth: usize,
        callback: &mut F,
    )
    where
        F: FnMut(&mut Node, Mat4, usize),
    {
        let world_transform = parent_transform * node.transform.to_matrix();

        callback(node, world_transform, depth);

        for child in &mut node.children {
            Self::traverse_node_mut(
                child,
                world_transform,
                depth + 1,
                callback,
            );
        }
    }
}