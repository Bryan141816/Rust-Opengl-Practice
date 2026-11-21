use crate::engine::{asset::AssetStorage, scene::resources::{Mesh, MeshHandle, Material, MaterialHandle}};
use crate::engine::{texture::Texture,texture::TextureHandle, shader::ShaderId, shader::ShaderSource};

pub struct AssetManager {
    meshes: AssetStorage<Mesh, MeshHandle>,
    materials: AssetStorage<Material, MaterialHandle>,
    textures: AssetStorage<Texture, TextureHandle>,
    shaders: AssetStorage<ShaderSource, ShaderId>,
}
impl AssetManager {
    pub fn new() -> Self {
        Self {
            meshes: AssetStorage::new(),
            materials: AssetStorage::new(),
            textures: AssetStorage::new(),
            shaders: AssetStorage::new(),
        }
    }
}
impl AssetManager {
    pub fn add_mesh(&mut self, mesh: Mesh) -> MeshHandle {
        MeshHandle(self.meshes.insert(mesh))
    }

    pub fn add_material(&mut self, material: Material) -> MaterialHandle {
        MaterialHandle(self.materials.insert(material))
    }

    pub fn add_texture(&mut self, texture: Texture) -> TextureHandle {
        TextureHandle(self.textures.insert(texture))
    }

    pub fn add_shader(&mut self, shader: ShaderSource) -> ShaderId {
        ShaderId(self.shaders.insert(shader))
    }
}
impl AssetManager {
    pub fn get_mesh(&self, handle: MeshHandle) -> Option<&Mesh> {
        self.meshes.get(handle.0)
    }

    pub fn get_material(&self, handle: MaterialHandle) -> Option<&Material> {
        self.materials.get(handle.0)
    }

    pub fn get_texture(&self, handle: TextureHandle) -> Option<&Texture> {
        self.textures.get(handle.0)
    }

    pub fn get_shader(&self, handle: ShaderId) -> Option<&ShaderSource> {
        self.shaders.get(handle.0)
    }
}