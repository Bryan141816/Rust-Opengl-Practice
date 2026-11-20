use crate::engine::scene::Scene;
use crate::engine::window::Window;

pub trait Application {
    fn update(
        &mut self,
        scene: &mut Scene,
        window: &mut dyn Window,
        delta_time: f32,
    );
}