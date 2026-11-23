use crate::engine::window::WindowConfig;

#[derive(Debug, Clone, Copy)]
pub enum GraphicsBackend {
    OpenGL,
    Vulkan,
}

pub struct EngineConfig {
    pub window: WindowConfig,
    pub graphics_backend: GraphicsBackend,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            graphics_backend: GraphicsBackend::OpenGL,
        }
    }
}