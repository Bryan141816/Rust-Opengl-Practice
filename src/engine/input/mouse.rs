#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseCursor {
    Normal,
    Hidden,
    Disabled,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct MousePosition {
    pub x: f64,
    pub y: f64,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct MouseDelta {
    pub x: f64,
    pub y: f64,
}