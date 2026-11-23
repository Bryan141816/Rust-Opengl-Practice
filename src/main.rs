
mod engine;
mod application;

use glam::{Quat, Vec3};
use gltf::json::extensions::{material, texture};

use crate::engine::{Engine, EngineConfig, primitive::Primitive, scene::{Component, Node, Scene, components::{Light, MeshInstance}, traversal::SceneTraversal}};
use crate::engine::scene::resources::Material;
use crate::engine::scene::components::Camera;

use crate::engine::scene::Transform;
use crate::application::MyApp;

fn main() {
    let engine_config = EngineConfig::default();
    let mut engine = Engine::new(&engine_config);
    engine.window.set_cursor_mode(engine::input::MouseCursor::Disabled);
    

    let cube = engine.create_mesh(Primitive::Cube { size: 0.5 });
    let plane = engine.create_mesh(Primitive::Plane { width: 10.0, height: 10.0 });
    let shader = engine.load_shader(    "shaders/basic.vert","shaders/basic.frag",);
    
    let texture = engine.load_texture("assets/textures/crate.jpg");
    let mut material = Material::new(shader); 
    let mut material2 = Material::new(shader);
    material.set_diffuse_texture(texture);
    
    let material_handle = engine.load_material(material);
    let material_handle_2 = engine.load_material(material2);
    let scene = Scene::new("Camera")
        .node(|root| {
            root.child("Cube", |cube_node| {
                cube_node
                    .transform(
                        Transform::new()
                            .position(Vec3::new(0.0, 0.0, 0.0)),
                    )
                    .component(Component::mesh(
                        "Cube",
                        MeshInstance::new(cube, material_handle),
                    ));
            });
            root.child("Plane", |plane_node|{
                plane_node
                    .transform(
                        Transform::new()
                            .position(Vec3::new(0.0, -0.5, 0.0)),
                    )
                    .component(Component::mesh("Plane", MeshInstance::new(plane, material_handle_2)));
            });
            root.child("MainLight", |light_node| {
                light_node
                    .transform(
                        Transform::new()
                            .position(Vec3::new(0.0, 5.0, 0.0))
                            .rotation(Quat::from_rotation_x(-90.0_f32.to_radians())),
                    )
                    .component(Component::light(
                        "MainLight",
                        Light::directional(Vec3::ONE, 1.0),
                    ));
            });
            root.child("Camera", |camera_node| {
                camera_node
                    .transform(
                        Transform::new()
                            .position(Vec3::new(0.0, 1.0, 2.0))
                            .rotation(Quat::from_rotation_x(-14.0_f32.to_radians())),
                    )
                    .component(Component::camera(
                        "Camera",
                        Camera::new(
                            engine_config.window.width as f32
                                / engine_config.window.height as f32,
                        ),
                    ))
                    .component(Component::light(
                        "Camera Light",
                        Light::spot(
                            Vec3::new(1.0, 0.0, 0.0),
                            2.0,
                            12.5,
                            17.5,
                        ),
                ));
            });
        })
        .build();

    engine.run(MyApp, scene);
}