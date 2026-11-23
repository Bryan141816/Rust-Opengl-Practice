use crate::engine::scene::{Component, Transform};

pub type NodeId = usize;
pub type NodeName = String;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Node {
    pub id: NodeId,
    pub name: NodeName,
    pub parent: Option<NodeId>,
    pub transform: Transform,
    pub components: Vec<Component>,
    pub children: Vec<NodeId>
}
impl Node {
    pub fn new(id: usize, name: impl  Into<String>, parent: Option<NodeId>) -> Self {
        Self {
            id: id,
            name: name.into(),
            parent,
            transform: Transform::default(),
            components: Vec::new(),
            children: Vec::new()
        }
    }
}