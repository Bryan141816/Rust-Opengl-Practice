mod engine;
mod application;

use glam::{Mat4, Vec3};

use crate::engine::engine::Engine;
use crate::engine::config::EngineConfig;
use crate::engine::camera::Camera;
use crate::engine::shape::Shape;
use crate::engine::texture::Texture;
use crate::engine::shader::{ShaderId, ShaderSource};
use crate::engine::lighting::{Light, DirectionalLight, PointLight, Material};
use crate::engine::scene::Scene;
use crate::engine::object::RenderObject;
use crate::application::MyApp;

fn main() {
    let mut engine = Engine::new(EngineConfig::default());
    
    let sphere_mesh = Shape::Sphere {
        radius: 1.0,
        sectors: 36,
        stacks: 18,
    }.generate();

    let cube_mesh = Shape::Cube { size: 0.5 }.generate();

    let sphere_handle = engine.create_mesh(&sphere_mesh);
    let cube_handle = engine.create_mesh(&cube_mesh);

    let texture_data = Texture::load("assets/textures/crate.jpg").unwrap();
    let texture_handle = engine.create_texture(&texture_data);

    let shader_source = ShaderSource::load(
        "shaders/basic.vert",
        "shaders/basic.frag",
    ).expect("Failed to load shader");

    engine.load_shader(ShaderId::new("basic"), &shader_source);

    let mut scene = Scene::new(
        Camera::new(
            Vec3::new(0.0, 0.0, 3.0),
            800.0 / 600.0,
        ),
    );

    scene.lights.add(Light::Directional(DirectionalLight {
        direction: Vec3::new(-0.2, -1.0, -0.3),
        color: Vec3::ONE,
        intensity: 1.0,
    }));

    scene.lights.add(Light::Point(PointLight {
        position: Vec3::new(2.0, 2.0, 2.0),
        color: Vec3::new(1.0, 0.0, 0.0),
        intensity: 2.0,
        constant: 1.0,
        linear: 0.09,
        quadratic: 0.032,
    }));

    scene.lights.add(Light::Point(PointLight {
        position: Vec3::new(-2.0, 1.0, 0.0),
        color: Vec3::new(0.0, 0.0, 1.0),
        intensity: 2.0,
        constant: 1.0,
        linear: 0.09,
        quadratic: 0.032,
    }));

    let mut material = Material::new(ShaderId::new("basic"));
    material.set_diffuse_texture(texture_handle);

    scene.add_object(RenderObject {
        mesh: sphere_handle,
        material: material.clone(),
        transform: Mat4::IDENTITY,
    });

    scene.add_object(RenderObject {
        mesh: cube_handle,
        material,
        transform: Mat4::from_translation(Vec3::new(2.0, 0.0, 5.0)),
    });

    engine.run(MyApp,scene);
}