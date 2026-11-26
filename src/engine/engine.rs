

use std::path::Path;

use glam::Vec3;

use crate::engine::asset::AssetManager;
use crate::engine::backend::factory::create_backend;
use crate::engine::config::EngineConfig;
use crate::engine::scene::resources::{Material, MaterialHandle, MeshHandle};
use super::window::Window;
use super::backend::renderer::Renderer;
use super::scene::Scene;
use super::shader::{ShaderId, ShaderSource};
use super::texture::{Texture, TextureHandle};
use super::time::Time;
use super::application::Application;
use crate::engine::primitive::Primitive;
use crate::engine::asset::obj::LoadObj;
pub struct Engine {
    pub window: Box<dyn Window>,
    renderer: Box<dyn Renderer>,
    time: Time,
    assets: AssetManager
}

impl  Engine {
    pub fn new(config: &EngineConfig) -> Self {
        let backend = create_backend(&config);

        Self {
            window: backend.window,
            renderer: backend.renderer,
            time: Time::new(),
            assets: AssetManager::new()
        }
    }
}
impl Engine {
    pub fn run<A:Application>(&mut self, mut app:A, mut scene: Scene) {

        while !self.window.should_close() {
            self.time.update();

            let delta_time= self.time.delta_time();
            
            self.window.poll_events();

            app.update(
                &mut scene,
                self.window.as_mut(),
                delta_time,
            );

            self.renderer.begin_frame();
            self.renderer.render(&scene, &self.assets);

            self.window.swap_buffers();
        }
        self.renderer.destroy();
    }
}
impl Engine {
    pub fn create_mesh(&mut self, primitive: Primitive) -> MeshHandle {
        let mesh = primitive.generate();
        
        let (handle, mesh) = self.assets.add_mesh(mesh);
        self.renderer.upload_mesh(handle, mesh);
        handle
    }
    pub fn import_obj_model(&mut self, path: &str) -> Vec<(MeshHandle, MaterialHandle)>  {
        let loaded = LoadObj::load_obj(path).unwrap();
        let default_material_handle = self.load_material(Material::new());
        let mut handles:Vec<(MeshHandle, MaterialHandle)> = Vec::new(); 
        for model in loaded.models{
            let (mesh_handle, mesh) = self.assets.add_mesh(model.mesh);
            self.renderer.upload_mesh(mesh_handle, mesh);
            if let Some(material) = model.material {
                let mut texture_handle:Option<TextureHandle> = None;
                if let Some(texture) = material.diffuse_map {
                    let path = Path::new(path);

                    let full_path = path
                        .parent()
                        .unwrap()
                        .join(texture);

                    let full_path_str = full_path.to_str().unwrap();
                    texture_handle = Some(self.load_texture(full_path_str));
                }
                let mut engine_material = Material::new();
                engine_material.diffuse_texture = texture_handle;
                engine_material.ambient = material.ambient
                        .map(Vec3::from_array)
                        .unwrap_or(engine_material.ambient);
                engine_material.diffuse = material.diffuse
                        .map(Vec3::from_array)
                        .unwrap_or(engine_material.diffuse);
                engine_material.diffuse = material.diffuse
                        .map(Vec3::from_array)
                        .unwrap_or(engine_material.diffuse);
                engine_material.specular = material.specular
                        .map(Vec3::from_array)
                        .unwrap_or(engine_material.specular);
                engine_material.shininess = material.shininess.unwrap_or(engine_material.shininess);

                let mat_handle = self.load_material(engine_material);
                handles.push((mesh_handle, mat_handle));
            }
            else {
                handles.push((mesh_handle, default_material_handle));
            }
        }
        handles
        // println!("{:#?}", &mesh);
    }

    pub fn load_texture(&mut self, path: &str) -> TextureHandle {
        let texture = Texture::load(path).unwrap();
        let (handle, texture) = self.assets.add_texture(texture);
        self.renderer.upload_texture(handle, &texture);
        handle
    }

    pub fn load_shader(&mut self,vertex_path: &str, fragment_path: &str) -> ShaderId {
        let source = ShaderSource::load(vertex_path, fragment_path).unwrap();
        let (id, shader) = self.assets.add_shader(source);
        self.renderer.upload_shader(id, &shader);
        id
    }
    pub fn load_material(&mut self, material: Material) -> MaterialHandle {
        let handle = self.assets.add_material(material);
        handle
    }
}

