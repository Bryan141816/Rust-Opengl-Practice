use crate::engine::scene::{Component, Transform};

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub transform: Transform,
    pub components: Vec<Component>,
    pub children: Vec<Node>
}
impl Node {
    pub fn new(name: impl  Into<String>) -> Self {
        Self {
            name: name.into(),
            transform: Transform::default(),
            components: Vec::new(),
            children: Vec::new()
        }
    }
    pub fn children(&mut self, children: impl IntoIterator<Item = Node>) -> &mut Self {
        self.children.extend(children);
        self
    }
    pub fn components(mut self, component: impl IntoIterator<Item = Component>) -> Self {
        self.components.extend(component);
        self
    }
    pub fn add_components(&mut self, component: Component) 
    {
        self.components.push(component);
    }
    pub fn add_children(&mut self, children: Node)
    {
        self.children.push(children);
    }
    pub fn transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}