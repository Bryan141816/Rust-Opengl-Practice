use crate::engine::window::Window;
use crate::engine::config::{EngineConfig, GraphicsBackend};
use super::renderer::Renderer;
pub struct EngineBackend {
    pub window: Box<dyn Window>,
    pub renderer: Box<dyn Renderer>,
}

pub fn create_backend(
    config: &EngineConfig,
) -> EngineBackend {
    match config.graphics_backend {
        GraphicsBackend::OpenGL => {
            crate::engine::backend::opengl::create(config)
        },
        GraphicsBackend::Vulkan => {
            panic!("Vulkan backend is not implemented yet or idk i don't want to code in vulkan.");
        }
    }
}