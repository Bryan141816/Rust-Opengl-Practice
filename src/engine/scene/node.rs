use crate::engine::scene::{Component, Transform};

pub struct Node {
    name: String,
    transform: Transform,
    components: Vec<Component>,
    children: Vec<Node>
}
impl Node {
    pub fn new(name: String) -> Self {
        Self {
            name,
            transform: Transform::default(),
            components: Vec::new(),
            children: Vec::new()
        }
    }
    pub fn components<I>(&mut self, component: I)
    where I: IntoIterator<Item = Component>,
     {
        self.components.extend(component);
    }
    pub fn children<I>(&mut self, children: I) 
    where I: IntoIterator<Item = Node>,
    {
        self.children.extend(children);
    }
}