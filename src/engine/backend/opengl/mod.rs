mod mesh;
mod shader;
mod texture;
mod renderer;
mod light_uniforms;
pub use shader::*;
pub use renderer::*;

use crate::engine::backend::factory::EngineBackend;
use crate::engine::config::EngineConfig;

use crate::engine::window::GlfwWindow;
use crate::engine::backend::opengl::OpenGLRendererBackend;

pub fn create(config: &EngineConfig) -> EngineBackend {
    let window = GlfwWindow::new(
        config.window_width,
        config.window_height,
        &config.window_title,
    );

    let gl = unsafe {
        glow::Context::from_loader_function_cstr(|s| {
            window.get_proc_address(s)
        })
    };

    let renderer = unsafe {
        OpenGLRendererBackend::new(gl)
    };

    EngineBackend {
        window: Box::new(window),
        renderer: Box::new(renderer),
    }
}