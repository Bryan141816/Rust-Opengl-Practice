use crate::engine::asset::AssetManager;
use crate::engine::scene::Scene;
use crate::engine::scene::resources::{Mesh, MeshHandle};
use crate::engine::texture::{Texture, TextureHandle};
use crate::engine::shader::{ShaderId, ShaderSource};
pub trait Renderer {
    fn begin_frame(&mut self);
    fn render(&mut self, scene: &Scene, assets: &AssetManager);
    fn destroy(&mut self);
    fn upload_mesh(&mut self, handle: MeshHandle, mesh: &Mesh);
    fn upload_texture(&mut self, handle: TextureHandle, texture: &Texture);
    fn upload_shader(&mut self, id: ShaderId, source: &ShaderSource);
}