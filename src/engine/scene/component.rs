use crate::engine::scene::components::{Camera, Light, MeshInstance};


pub enum Component {
    Mesh(MeshInstance),
    Light(Light),
    Camera(Camera)
}