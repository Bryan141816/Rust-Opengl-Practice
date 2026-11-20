use glfw::{
    fail_on_errors,
    Context,
    Glfw,
    GlfwReceiver,
    OpenGlProfileHint,
    PWindow,
    WindowHint,
    WindowMode,
    WindowEvent,
    Key as GlfwKey,
    Action,
    MouseButton as GlfwMouseButton,
    CursorMode,
};
use crate::engine::{input::{
    Key as EngineKey, MouseButton, MouseCursor, MouseDelta, MousePosition,
}, window::WindowConfig};
use std::ffi::{CStr, c_void};
use super::window::Window;
#[allow(dead_code)]
pub struct GlfwWindow {
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,

    mouse_position: MousePosition,
    mouse_delta: MouseDelta,
    last_mouse_position: MousePosition,
    first_mouse: bool,
}
#[allow(dead_code)]
impl GlfwWindow {
pub fn new(config: &WindowConfig) -> Self {
        let mut glfw = glfw::init(fail_on_errors!())
            .expect("Failed to initialize GLFW");

        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(
            WindowHint::OpenGlProfile(OpenGlProfileHint::Core),
        );

        let (mut window, events) = glfw
            .create_window(
                config.width,
                config.height,
                &config.title,
                WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        window.make_current();

        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_resizable(config.resizable);

            match config.cursor_mode {
                MouseCursor::Normal => window.set_cursor_mode(glfw::CursorMode::Normal),
                MouseCursor::Hidden => window.set_cursor_mode(glfw::CursorMode::Hidden),
                MouseCursor::Disabled => window.set_cursor_mode(glfw::CursorMode::Disabled),
            }

        let (x, y) = window.get_cursor_pos();

        glfw.set_swap_interval(if config.vsync {
            glfw::SwapInterval::Sync(1)
        } else {
            glfw::SwapInterval::None
        });

        Self {
            glfw,
            window,
            events,

            mouse_position: MousePosition { x, y },
            mouse_delta: MouseDelta { x: 0.0, y: 0.0 },
            last_mouse_position: MousePosition { x, y },
            first_mouse: true,
        }
    }
    pub fn raw(&self) -> &PWindow {
        &self.window
    }

    pub fn raw_mut(&mut self) -> &mut PWindow {
        &mut self.window
    }

    pub fn glfw(&mut self) -> &mut Glfw {
        &mut self.glfw
    }

    pub fn events(&self) -> &GlfwReceiver<(f64, WindowEvent)> {
        &self.events
    }
    pub fn get_proc_address(&self, name: &CStr) -> *const c_void {
        self.glfw
            .get_proc_address_raw(name.to_str().unwrap())
            .map_or(std::ptr::null(), |f| {
                f as *const () as *const c_void
        })
    }

}
fn map_key(key: EngineKey) -> GlfwKey {
    match key {
        EngineKey::A => GlfwKey::A,
        EngineKey::B => GlfwKey::B,
        EngineKey::C => GlfwKey::C,
        EngineKey::D => GlfwKey::D,
        EngineKey::E => GlfwKey::E,
        EngineKey::F => GlfwKey::F,
        EngineKey::G => GlfwKey::G,
        EngineKey::H => GlfwKey::H,
        EngineKey::I => GlfwKey::I,
        EngineKey::J => GlfwKey::J,
        EngineKey::K => GlfwKey::K,
        EngineKey::L => GlfwKey::L,
        EngineKey::M => GlfwKey::M,
        EngineKey::N => GlfwKey::N,
        EngineKey::O => GlfwKey::O,
        EngineKey::P => GlfwKey::P,
        EngineKey::Q => GlfwKey::Q,
        EngineKey::R => GlfwKey::R,
        EngineKey::S => GlfwKey::S,
        EngineKey::T => GlfwKey::T,
        EngineKey::U => GlfwKey::U,
        EngineKey::V => GlfwKey::V,
        EngineKey::W => GlfwKey::W,
        EngineKey::X => GlfwKey::X,
        EngineKey::Y => GlfwKey::Y,
        EngineKey::Z => GlfwKey::Z,

        EngineKey::Num0 => GlfwKey::Num0,
        EngineKey::Num1 => GlfwKey::Num1,
        EngineKey::Num2 => GlfwKey::Num2,
        EngineKey::Num3 => GlfwKey::Num3,
        EngineKey::Num4 => GlfwKey::Num4,
        EngineKey::Num5 => GlfwKey::Num5,
        EngineKey::Num6 => GlfwKey::Num6,
        EngineKey::Num7 => GlfwKey::Num7,
        EngineKey::Num8 => GlfwKey::Num8,
        EngineKey::Num9 => GlfwKey::Num9,

        EngineKey::F1 => GlfwKey::F1,
        EngineKey::F2 => GlfwKey::F2,
        EngineKey::F3 => GlfwKey::F3,
        EngineKey::F4 => GlfwKey::F4,
        EngineKey::F5 => GlfwKey::F5,
        EngineKey::F6 => GlfwKey::F6,
        EngineKey::F7 => GlfwKey::F7,
        EngineKey::F8 => GlfwKey::F8,
        EngineKey::F9 => GlfwKey::F9,
        EngineKey::F10 => GlfwKey::F10,
        EngineKey::F11 => GlfwKey::F11,
        EngineKey::F12 => GlfwKey::F12,
        EngineKey::F13 => GlfwKey::F13,
        EngineKey::F14 => GlfwKey::F14,
        EngineKey::F15 => GlfwKey::F15,
        EngineKey::F16 => GlfwKey::F16,
        EngineKey::F17 => GlfwKey::F17,
        EngineKey::F18 => GlfwKey::F18,
        EngineKey::F19 => GlfwKey::F19,
        EngineKey::F20 => GlfwKey::F20,
        EngineKey::F21 => GlfwKey::F21,
        EngineKey::F22 => GlfwKey::F22,
        EngineKey::F23 => GlfwKey::F23,
        EngineKey::F24 => GlfwKey::F24,
        EngineKey::F25 => GlfwKey::F25,

        EngineKey::Escape => GlfwKey::Escape,
        EngineKey::Tab => GlfwKey::Tab,
        EngineKey::CapsLock => GlfwKey::CapsLock,
        EngineKey::Backspace => GlfwKey::Backspace,
        EngineKey::Enter => GlfwKey::Enter,
        EngineKey::Insert => GlfwKey::Insert,
        EngineKey::Delete => GlfwKey::Delete,
        EngineKey::Home => GlfwKey::Home,
        EngineKey::End => GlfwKey::End,
        EngineKey::PageUp => GlfwKey::PageUp,
        EngineKey::PageDown => GlfwKey::PageDown,

        EngineKey::Up => GlfwKey::Up,
        EngineKey::Down => GlfwKey::Down,
        EngineKey::Left => GlfwKey::Left,
        EngineKey::Right => GlfwKey::Right,

        EngineKey::LeftShift => GlfwKey::LeftShift,
        EngineKey::RightShift => GlfwKey::RightShift,
        EngineKey::LeftControl => GlfwKey::LeftControl,
        EngineKey::RightControl => GlfwKey::RightControl,
        EngineKey::LeftAlt => GlfwKey::LeftAlt,
        EngineKey::RightAlt => GlfwKey::RightAlt,
        EngineKey::LeftSuper => GlfwKey::LeftSuper,
        EngineKey::RightSuper => GlfwKey::RightSuper,

        EngineKey::Space => GlfwKey::Space,
        EngineKey::Apostrophe => GlfwKey::Apostrophe,
        EngineKey::Comma => GlfwKey::Comma,
        EngineKey::Minus => GlfwKey::Minus,
        EngineKey::Period => GlfwKey::Period,
        EngineKey::Slash => GlfwKey::Slash,
        EngineKey::Semicolon => GlfwKey::Semicolon,
        EngineKey::Equal => GlfwKey::Equal,
        EngineKey::LeftBracket => GlfwKey::LeftBracket,
        EngineKey::Backslash => GlfwKey::Backslash,
        EngineKey::RightBracket => GlfwKey::RightBracket,
        EngineKey::GraveAccent => GlfwKey::GraveAccent,

        EngineKey::NumLock => GlfwKey::NumLock,
        EngineKey::ScrollLock => GlfwKey::ScrollLock,
        EngineKey::PrintScreen => GlfwKey::PrintScreen,
        EngineKey::Pause => GlfwKey::Pause,

        EngineKey::Keypad0 => GlfwKey::Kp0,
        EngineKey::Keypad1 => GlfwKey::Kp1,
        EngineKey::Keypad2 => GlfwKey::Kp2,
        EngineKey::Keypad3 => GlfwKey::Kp3,
        EngineKey::Keypad4 => GlfwKey::Kp4,
        EngineKey::Keypad5 => GlfwKey::Kp5,
        EngineKey::Keypad6 => GlfwKey::Kp6,
        EngineKey::Keypad7 => GlfwKey::Kp7,
        EngineKey::Keypad8 => GlfwKey::Kp8,
        EngineKey::Keypad9 => GlfwKey::Kp9,
        EngineKey::KeypadDecimal => GlfwKey::KpDecimal,
        EngineKey::KeypadDivide => GlfwKey::KpDivide,
        EngineKey::KeypadMultiply => GlfwKey::KpMultiply,
        EngineKey::KeypadSubtract => GlfwKey::KpSubtract,
        EngineKey::KeypadAdd => GlfwKey::KpAdd,
        EngineKey::KeypadEnter => GlfwKey::KpEnter,
        EngineKey::KeypadEqual => GlfwKey::KpEqual,

        EngineKey::Menu => GlfwKey::Menu,
    }
}
fn map_mouse_button(button: MouseButton) -> GlfwMouseButton {
    match button {
        MouseButton::Left => GlfwMouseButton::Button1,
        MouseButton::Right => GlfwMouseButton::Button2,
        MouseButton::Middle => GlfwMouseButton::Button3,
        MouseButton::Button4 => GlfwMouseButton::Button4,
        MouseButton::Button5 => GlfwMouseButton::Button5,
        MouseButton::Button6 => GlfwMouseButton::Button6,
        MouseButton::Button7 => GlfwMouseButton::Button7,
        MouseButton::Button8 => GlfwMouseButton::Button8,
    }
}

fn map_cursor_mode(mode: MouseCursor) -> CursorMode {
    match mode {
        MouseCursor::Normal => CursorMode::Normal,
        MouseCursor::Hidden => CursorMode::Hidden,
        MouseCursor::Disabled => CursorMode::Disabled,
    }
}
impl Window for GlfwWindow {
    fn should_close(&self) -> bool {
        self.window.should_close()
    }

    fn set_should_close(&mut self, value: bool) {
        self.window.set_should_close(value);
    }

    fn poll_events(&mut self) {
        self.glfw.poll_events();

        let (x, y) = self.window.get_cursor_pos();

        self.mouse_position = MousePosition { x, y };

        if self.first_mouse {
            self.last_mouse_position = self.mouse_position;
            self.mouse_delta = MouseDelta { x: 0.0, y: 0.0 };
            self.first_mouse = false;
        } else {
            self.mouse_delta = MouseDelta {
                x: x - self.last_mouse_position.x,
                y: self.last_mouse_position.y - y,
            };

            self.last_mouse_position = self.mouse_position;
        }
    }
    

    fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    fn size(&self) -> (u32, u32) {
        let (width, height) = self.window.get_size();
        (width as u32, height as u32)
    }

    fn is_key_pressed(&self, key: EngineKey) -> bool {
        self.window.get_key(map_key(key)) == Action::Press
    }

    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.window.get_mouse_button(map_mouse_button(button)) == Action::Press
    }

    fn mouse_position(&self) -> MousePosition {
        self.mouse_position
    }
    fn mouse_delta(&self) -> MouseDelta {
        self.mouse_delta
    }

    fn set_cursor_mode(&mut self, mode: MouseCursor) {
        self.window.set_cursor_mode(map_cursor_mode(mode));

        self.first_mouse = true;
    }

}
