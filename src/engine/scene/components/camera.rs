#[derive(Debug)]
pub struct Camera {
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            fov: 75.0_f32.to_radians(),
            aspect_ratio,
            near_plane: 0.1,
            far_plane: 100.0,
        }
    }

    pub fn set_aspect_ratio(&mut self, width: u32, height: u32) {
        if height != 0 {
            self.aspect_ratio = width as f32 / height as f32;
        }
    }
}