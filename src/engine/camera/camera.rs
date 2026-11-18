use glam::{Mat4, Vec3};
use glam::camera::rh;

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
    pub sensitivity: f32
}
impl Camera {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            yaw: -90.0_f32.to_radians(),
            pitch: 0.0,
            speed: 3.0,
            sensitivity: 0.1,
        }
    }

    pub fn direction(&self) -> Vec3 {
        Vec3::new(
            self.yaw.cos() * self.pitch.cos(), 
            self.pitch.sin(), 
            self.yaw.sin() * self.pitch.cos()
        ).normalize()
    }
    pub fn view_matrix(&self) -> Mat4 {
        rh::view::look_at_mat4(
            self.position,
            self.position + self.direction(),
            Vec3::Y,
        )
    }

}
