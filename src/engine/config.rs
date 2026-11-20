#[derive(Debug, Clone, Copy)]
pub enum GraphicsBackend {
    OpenGL,
    Vulkan,
}

pub struct EngineConfig {
    pub window_width: u32,
    pub window_height: u32,
    pub window_title: String,
    pub graphics_backend: GraphicsBackend,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window_width: 800,
            window_height: 600,
            window_title: "My App".to_string(),
            graphics_backend: GraphicsBackend::OpenGL,
        }
    }
}