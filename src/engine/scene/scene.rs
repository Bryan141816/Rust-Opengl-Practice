use crate::engine::scene::Node;

pub struct Scene {
    root: Node
}
impl Default for Scene {
    fn default() -> Self {
        Self {
            root: Node::new("Root".into()),
        }   
    }
}
impl Scene {
    pub fn children<I>(&mut self, children: I)
    where I: IntoIterator<Item = Node>,
    {
        self.root.children(children);
    }
}