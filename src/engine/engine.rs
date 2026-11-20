use crate::engine::backend::factory::create_backend;
use crate::engine::config::EngineConfig;

use super::window::Window;
use super::backend::renderer::Renderer;
use super::scene::Scene;
use super::mesh::{Mesh, MeshHandle};
use super::shader::{ShaderId, ShaderSource};
use super::texture::{Texture, TextureHandle};
use super::time::Time;
use super::application::Application;
pub struct Engine {
    window: Box<dyn Window>,
    renderer: Box<dyn Renderer>,
    time: Time,
}

impl  Engine {
    pub fn new(config: EngineConfig) -> Self {
        let backend = create_backend(&config);

        Self {
            window: backend.window,
            renderer: backend.renderer,
            time: Time::new(),
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
            self.renderer.render(&scene);

            self.window.swap_buffers();
        }
        self.renderer.destroy();
    }
}
impl Engine {
    pub fn create_mesh(&mut self, mesh: &Mesh) -> MeshHandle {
        self.renderer.upload_mesh(mesh)
    }

    pub fn create_texture(&mut self, texture: &Texture) -> TextureHandle {
        self.renderer.upload_texture(texture)
    }

    pub fn load_shader(&mut self, id: ShaderId, source: &ShaderSource) {
        self.renderer.load_shader(id, source);
    }
}