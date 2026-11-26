use crate::engine::{scene::components::{Camera, Light, MeshInstance}, shader::ShaderId};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Component {
    Mesh {
        name: String,
        instance: MeshInstance,
        shader: ShaderId,
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
    pub fn mesh(name: impl Into<String>, instance: MeshInstance,shader: ShaderId,) -> Self {
        Self::Mesh {
            name: name.into(),
            instance,
            shader
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