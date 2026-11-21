use crate::engine::scene::Scene;
use crate::engine::scene::resources::{Mesh, MeshHandle};
use crate::engine::texture::{Texture, TextureHandle};
use crate::engine::shader::{ShaderId, ShaderSource};
pub trait Renderer {
    fn begin_frame(&mut self);
    fn render(&mut self, scene: &Scene);
    fn destroy(&mut self);
    fn upload_mesh(&mut self, mesh: &Mesh) -> MeshHandle;

    fn upload_texture(&mut self, texture: &Texture) -> TextureHandle;

    fn upload_shader(&mut self, source: &ShaderSource) -> ShaderId;
}