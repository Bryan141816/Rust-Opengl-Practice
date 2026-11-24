use glam::Vec3;

use crate::engine::{
    Application,
    input::Key,
    scene::Scene,
    window::Window,
};

const CAMERA_SPEED: f32 = 3.0;

pub struct MyApp;

impl Application for MyApp {
    fn update(
        &mut self,
        scene: &mut Scene,
        window: &mut dyn Window,
        delta_time: f32,
    ) {
        let cube = scene.find_node_mut("Cube").unwrap();

        let velocity = CAMERA_SPEED * delta_time;

        if window.is_key_pressed(Key::Escape) {
            window.set_should_close(true);
        }

        if window.is_key_pressed(Key::W) {
            cube.transform.position.z -= velocity;
        }

        if window.is_key_pressed(Key::S) {
            cube.transform.position.z += velocity;
        }

        if window.is_key_pressed(Key::A) {
            cube.transform.position.x -= velocity;
        }

        if window.is_key_pressed(Key::D) {
            cube.transform.position.x += velocity;
        }

        if window.is_key_pressed(Key::Space) {
            cube.transform.position.y += velocity;
        }

        if window.is_key_pressed(Key::LeftShift) {
            cube.transform.position.y -= velocity;
        }
    }
}