use glam::{EulerRot, Quat, Vec3};

use crate::engine::{
    Application,
    input::Key,
    scene::Scene,
    window::Window,
};

const CAMERA_SENSITIVITY: f32 = 0.1;
const CAMERA_SPEED: f32 = 3.0;


pub struct MyApp;

impl Application for MyApp {
    
    fn update(
        &mut self,
        scene: &mut Scene,
        window: &mut dyn Window,
        delta_time: f32,
    ) {
    let camera_id = scene.active_camera.clone();

    let camera = scene.find_node_mut(&camera_id).unwrap();

        let mouse = window.mouse_delta();

        let (mut yaw, mut pitch, _roll) = camera.transform.rotation.to_euler(EulerRot::YXZ);

        yaw -= mouse.x as f32 * CAMERA_SENSITIVITY.to_radians();
        pitch += mouse.y as f32 * CAMERA_SENSITIVITY.to_radians();

        pitch = pitch.clamp(
            -89.0_f32.to_radians(),
            89.0_f32.to_radians(),
        );



        let direction = camera.transform.direction();

        let right = direction
            .cross(Vec3::Y)
            .normalize();

        let velocity = 3.0 * delta_time;

        if window.is_key_pressed(Key::Escape) {
            window.set_should_close(true);
        }

        if window.is_key_pressed(Key::W) {
            camera.transform.position += direction * velocity;
        }

        if window.is_key_pressed(Key::S) {
            camera.transform.position -= direction * velocity;
        }

        if window.is_key_pressed(Key::A) {
            camera.transform.position -= right * velocity;
        }

        if window.is_key_pressed(Key::D) {
            camera.transform.position += right * velocity;
        }

        if window.is_key_pressed(Key::Space) {
            camera.transform.position.y += velocity;
        }

        if window.is_key_pressed(Key::LeftShift) {
            camera.transform.position.y -= velocity;
        }

        let new_rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
        camera.transform.rotation = new_rotation;
    }
}