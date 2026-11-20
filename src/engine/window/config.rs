use crate::engine::input::MouseCursor;

pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub resizable: bool,
    pub cursor_mode: MouseCursor,
    pub vsync: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Untitled".into(),
            width: 800,
            height: 600,
            fullscreen: false,
            resizable: false,
            cursor_mode: MouseCursor::Normal,
            vsync: false
        }
    }
}