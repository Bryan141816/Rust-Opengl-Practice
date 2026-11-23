use crate::engine::scene::components::{Camera, Light, MeshInstance};

#[derive(Debug)]
pub enum Component {
    Mesh {
        name: String,
        instance: MeshInstance,
    },
    Light {
        name: String,
        light: Light,
    },
    Camera {
        name: String,
        camera: Camera,
    },
}

impl Component {
    pub fn mesh(name: impl Into<String>, instance: MeshInstance) -> Self {
        Self::Mesh {
            name: name.into(),
            instance,
        }
    }

    pub fn light(name: impl Into<String>, light: Light) -> Self {
        Self::Light {
            name: name.into(),
            light,
        }
    }

    pub fn camera(name: impl Into<String>, camera: Camera) -> Self {
        Self::Camera {
            name: name.into(),
            camera,
        }
    }
}