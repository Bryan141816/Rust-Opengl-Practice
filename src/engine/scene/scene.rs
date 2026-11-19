use crate::engine::{
    camera::Camera,
    lighting::LightManager,
    object::RenderObject,
};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<RenderObject>,
    pub lights: LightManager,
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            objects: Vec::new(),
            lights: LightManager::new(),
        }
    }

    pub fn add_object(&mut self, object: RenderObject) {
        self.objects.push(object);
    }
}