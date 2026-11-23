
mod engine;
mod application;

use glam::{Quat, Vec3};

use crate::engine::{Engine, EngineConfig, primitive::Primitive, scene::{Component, Node, Scene, components::{Light, MeshInstance}, traversal::SceneTraversal}};
use crate::engine::scene::resources::Material;
use crate::engine::scene::components::Camera;

use crate::engine::scene::Transform;
use crate::application::MyApp;

fn main() {
    let engine_config = EngineConfig::default();
    let mut engine = Engine::new(&engine_config);

    let cube = engine.create_mesh(Primitive::Cube { size: 0.5 });
    let shader = engine.load_shader(    "shaders/basic.vert","shaders/basic.frag",);
    let material = engine.load_material(Material::new(shader));

    let mut scene = Scene::new("Camera");
    scene.root.children([
        Node::new("Cube")
            .transform(
                Transform::new()
                    .position(Vec3::new(0.0, 0.0, 0.0))
            )
            .components([
                Component::mesh(
                    "Cube",
                    MeshInstance::new(cube, material)
                ),
            ]),

        Node::new("MainLight")
            .transform(
                Transform::new()
                    .position(Vec3::new(3.0, 5.0, 3.0))
                    .rotation(Quat::from_rotation_x(-10.0_f32.to_radians()))
            )
            .components([
                Component::light(
                    "MainLight",
                    Light::directional(Vec3::ONE, 0.5)
                ),
            ]),

       Node::new("Camera")
            .transform(
                Transform::new()
                    .position(Vec3::new(0.0, 1.0, 2.0))
                    .rotation(
                        Quat::from_rotation_x(-14.0_f32.to_radians())
                    )
            )
            .components([
                Component::camera(
                    "Camera",
                    Camera::new(
                        engine_config.window.width as f32
                            / engine_config.window.height as f32
                    )
                ),
            ]),
    ]);


    engine.run(MyApp, scene);
}