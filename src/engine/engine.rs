use std::collections::HashMap;

use gltf::json::extensions::{mesh, texture};

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
pub struct Engine {
    window: Box<dyn Window>,
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
        let (handle) = self.assets.add_material(material);
        handle
    }
}

