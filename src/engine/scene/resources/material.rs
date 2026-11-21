use glam::Vec3;
use crate::engine::{
    shader::ShaderId,
    texture::TextureHandle,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialHandle(pub u32);

#[derive(Clone)]
pub struct Material {
    pub shader: ShaderId,
    pub diffuse_texture: Option<TextureHandle>,
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub shininess: f32,
}
impl Material {
    pub fn new(shader: ShaderId) -> Self {
        Self {
            shader,
            diffuse_texture: None,

            ambient: Vec3::splat(0.15),
            diffuse: Vec3::ONE,
            specular: Vec3::ONE,
            shininess: 32.0,
        }
    }
    pub fn set_diffuse_texture(&mut self, texture_handle: TextureHandle) {
        self.diffuse_texture = Some(texture_handle);
    }
}