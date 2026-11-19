use super::Light;

pub struct LightManager {
    pub lights: Vec<Light>,
}

impl LightManager {
    pub fn new() -> Self {
        Self {
            lights: Vec::new(),
        }
    }
    pub fn add(&mut self, light: Light) {
        self.lights.push(light);
    }
    pub fn clear(&mut self) {
        self.lights.clear();
    }
}
