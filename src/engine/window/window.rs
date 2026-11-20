use crate::engine::input::Key as EngineKey;
use crate::engine::input::{MouseButton, MousePosition, MouseDelta, MouseCursor};

pub trait Window {
    fn should_close(&self) -> bool;

    fn set_should_close(&mut self, value: bool);
    
    fn poll_events(&mut self);

    fn swap_buffers(&mut self);

    fn size(&self) -> (u32, u32);

    fn is_key_pressed(&self, key: EngineKey) -> bool;

    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool;
    fn mouse_position(&self) -> MousePosition;
    fn mouse_delta(&self) -> MouseDelta;
    fn set_cursor_mode(&mut self, mode: MouseCursor);
}
