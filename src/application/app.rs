use glam::Vec3;

use crate::engine::{
    application::Application,
    input::Key,
    scene::Scene,
    window::Window,
};

pub struct MyApp;

impl Application for MyApp {
    
    fn update(
        &mut self,
        scene: &mut Scene,
        window: &mut dyn Window,
        delta_time: f32,
    ) {
        let mouse = window.mouse_delta();

        scene.camera.yaw +=
            mouse.x as f32 * scene.camera.sensitivity.to_radians();

        scene.camera.pitch +=
            mouse.y as f32 * scene.camera.sensitivity.to_radians();

        scene.camera.pitch = scene.camera.pitch.clamp(
            -89.0_f32.to_radians(),
            89.0_f32.to_radians(),
        );

        let direction = scene.camera.direction();

        let right = direction
            .cross(Vec3::Y)
            .normalize();

        let velocity = scene.camera.speed * delta_time;

        if window.is_key_pressed(Key::Escape) {
            window.set_should_close(true);
        }

        if window.is_key_pressed(Key::W) {
            scene.camera.position += direction * velocity;
        }

        if window.is_key_pressed(Key::S) {
            scene.camera.position -= direction * velocity;
        }

        if window.is_key_pressed(Key::A) {
            scene.camera.position -= right * velocity;
        }

        if window.is_key_pressed(Key::D) {
            scene.camera.position += right * velocity;
        }

        if window.is_key_pressed(Key::Space) {
            scene.camera.position.y += velocity;
        }

        if window.is_key_pressed(Key::LeftShift) {
            scene.camera.position.y -= velocity;
        }
    }
}