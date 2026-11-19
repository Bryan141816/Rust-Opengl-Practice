use glam::{Vec2, Vec3};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
}