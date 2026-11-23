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
    pub fn add_mesh(&mut self, mesh: Mesh) -> (MeshHandle, &Mesh) {
        let handle = MeshHandle(self.meshes.insert(mesh));
        let mesh_ref = self.meshes.get(handle.0).unwrap();

        (handle, mesh_ref)
    }

    pub fn add_material(&mut self, material: Material) -> MaterialHandle {
        MaterialHandle(self.materials.insert(material))
    }

    pub fn add_texture(&mut self, texture: Texture) -> (TextureHandle, &Texture) {
        let handle = TextureHandle(self.textures.insert(texture));
        let texture_ref = self.textures.get(handle.0).unwrap();
        (handle, texture_ref)
    }

    pub fn add_shader(&mut self, shader: ShaderSource) -> (ShaderId, &ShaderSource) {
        let id = ShaderId(self.shaders.insert(shader));
        let shader_ref = self.shaders.get(id.0).unwrap();
        (id, shader_ref)
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