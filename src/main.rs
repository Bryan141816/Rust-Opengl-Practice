
mod engine;
mod application;

use glam::{Quat, Vec3};

use crate::engine::{Engine, EngineConfig, primitive::Primitive, scene::{Component, Scene, components::{Light, MeshInstance}}};
use crate::engine::scene::resources::Material;
use crate::engine::scene::components::Camera;

use crate::engine::scene::Transform;
use crate::application::MyApp;

fn main() {
    let engine_config = EngineConfig::default();
    let mut engine = Engine::new(&engine_config);
    engine.window.set_cursor_mode(engine::input::MouseCursor::Disabled);
    
    let loaded = engine.import_obj_model("assets/models/chibi-mambo-3d-model/source/model/base.obj");
    println!("{:#?}", loaded);
    let cube = engine.create_mesh(Primitive::Cube { size: 0.5 });
    let plane = engine.create_mesh(Primitive::Plane { width: 10.0, height: 10.0 });
    let shader = engine.load_shader(    "shaders/basic.vert","shaders/basic.frag",);
    
    let texture = engine.load_texture("assets/textures/crate.jpg");
    let mut material = Material::new(); 
    let material2 = Material::new();
    material.set_diffuse_texture(texture);
    
    let material_handle_2 = engine.load_material(material2);
    let scene = Scene::new("Camera")
        .node(|root| {
            root.child("Cube", |cube_node| {
                cube_node
                    .transform(
                        Transform::new()
                            .position(Vec3::new(0.0, 2.0, 0.0)),
                    )
                    // .component(Component::mesh(
                    //     "Cube",
                    //     MeshInstance::new(mambo, material_handle),
                    // ))
                    .child("Camera", |camera|{

                        camera.component(Component::camera("Camera", 
                            Camera::new(
                                engine_config.window.width as f32
                                    / engine_config.window.height as f32,
                            ),
                        )).transform(
                            Transform::new()
                                .position(Vec3::new(0.0, 1.0, 2.0))
                                .rotation(Quat::from_rotation_x(-14.0_f32.to_radians())),
                        );
                    });
            });
            root.child("Plane", |plane_node|{
                plane_node
                    .transform(
                        Transform::new()
                            .position(Vec3::new(0.0, -0.5, 0.0)),
                    )
                    .component(Component::mesh("Plane", MeshInstance::new(plane, material_handle_2), shader));
            });
            root.child("LightingRig", |rig| {
                let lights = [
                    // Front
                    (
                        "FrontLight",
                        Vec3::new(0.0, 0.0, 1.0),
                        Quat::from_rotation_y(180.0_f32.to_radians()),
                    ),

                    // Back
                    (
                        "BackLight",
                        Vec3::new(0.0, 0.0, -1.0),
                        Quat::IDENTITY,
                    ),

                    // Right
                    (
                        "RightLight",
                        Vec3::new(1.0, 0.0, 0.0),
                        Quat::from_rotation_y(-90.0_f32.to_radians()),
                    ),

                    // Left
                    (
                        "LeftLight",
                        Vec3::new(-1.0, 0.0, 0.0),
                        Quat::from_rotation_y(90.0_f32.to_radians()),
                    ),

                    // Top
                    (
                        "TopLight",
                        Vec3::new(0.0, 0.5, 0.0),
                        Quat::from_rotation_x(90.0_f32.to_radians()),
                    ),

                    // Bottom
                    (
                        "BottomLight",
                        Vec3::new(0.0, -1.0, 0.0),
                        Quat::from_rotation_x(-90.0_f32.to_radians()),
                    ),
                ];

                for (name, position, rotation) in lights {
                    rig.child(name, |light| {
                        light
                            .transform(
                                Transform::new()
                                    .position(position * 2.0)
                                    .rotation(rotation),
                            )
                            .component(Component::light(
                                name,
                                Light::directional(Vec3::ONE, 0.5),
                            ));
                    });
                }
            });
        })
        .build();

    engine.run(MyApp, scene);
}