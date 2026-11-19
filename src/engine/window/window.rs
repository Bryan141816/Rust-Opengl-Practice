use super::input::Key as EngineKey;

pub trait Window {
    fn should_close(&self) -> bool;

    fn set_should_close(&mut self, value: bool);
    
    fn poll_events(&mut self);

    fn swap_buffers(&mut self);

    fn size(&self) -> (u32, u32);

    fn is_key_pressed(&self, key: EngineKey) -> bool;
}
