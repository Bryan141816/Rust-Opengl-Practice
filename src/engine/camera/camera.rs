use glam::{Mat4, Vec3};
use glam::camera::rh;

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
    pub sensitivity: f32,

    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

impl Camera {
    pub fn new(position: Vec3, aspect_ratio: f32) -> Self {
        Self {
            position,
            yaw: -90.0_f32.to_radians(),
            pitch: 0.0,
            speed: 3.0,
            sensitivity: 0.1,

            fov: 75.0_f32.to_radians(),
            aspect_ratio,
            near_plane: 0.1,
            far_plane: 100.0,
        }
    }

    pub fn direction(&self) -> Vec3 {
        Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize()
    }

    pub fn view_matrix(&self) -> Mat4 {
        rh::view::look_at_mat4(
            self.position,
            self.position + self.direction(),
            Vec3::Y,
        )
    }

    pub fn set_aspect_ratio(&mut self, width: u32, height: u32) {
        if height != 0 {
            self.aspect_ratio = width as f32 / height as f32;
        }
    }
}