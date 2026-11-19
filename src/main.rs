mod engine;
extern crate glfw;

use std::time::Instant;


use glam::{Vec3, Mat4};

use crate::engine::window::{GlfwWindow, Window};

use crate::engine::camera::Camera;
use crate::engine::texture::Texture;
use crate::engine::shape::Shape;
use crate::engine::lighting::{DirectionalLight, Light, PointLight};
use crate::engine::lighting::Material;

use crate::engine::shader::{ShaderId, ShaderSource};
use crate::engine::window::Key;
use crate::engine::scene::Scene;
use crate::engine::object::RenderObject;
use crate::engine::backend::opengl::OpenGLRenderer;

const WINDOW_TITLE: &str = "My App";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let mut window = GlfwWindow::new(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE);

    let gl = unsafe {
        glow::Context::from_loader_function_cstr(|s| {
            window.glfw().get_proc_address_raw(s.to_str().unwrap())
                .map_or(
                    std::ptr::null(), 
                    |f| f as *const () as *const std::ffi::c_void
                )
        })
    };

    println!("OpenGL initialized");

    unsafe {
        let mut renderer = OpenGLRenderer::new(gl);
    
        let sphere_mesh= Shape::Sphere { radius: 1.0, sectors: 36, stacks: 18 }.generate();
        
        let sphere_handle = renderer.upload_mesh(&sphere_mesh);

        let texture_data = Texture::load("assets/textures/crate.jpg").unwrap();
        
        let texture_handle = renderer.upload_texture(&texture_data);

        let shader_source = ShaderSource::load(
            "shaders/basic.vert",
            "shaders/basic.frag",
        ).expect("Failed to create shader");
        
        renderer.load_shader(ShaderId::new("basic"), &shader_source);


        let mut scene = Scene::new(
            Camera::new(
                Vec3::new(0.0, 0.0, 3.0),
                WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
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

        let object = RenderObject {
            mesh: sphere_handle,
            material: material.clone(),
            transform: Mat4::IDENTITY,
        };

        scene.add_object(object);

        let object2 = RenderObject {
            mesh: sphere_handle,
            material: material.clone(),
            transform: Mat4::from_translation(Vec3::new(2.0, 0.0, 0.0)),
        };

        scene.add_object(object2);

        let mut last_frame = Instant::now();

        let mut last_mouse_x = 400.0;
        let mut last_mouse_y = 300.0;
        let mut first_mouse = true;
        while !window.should_close() {


            let current_frame = Instant::now();

            let delta_time = current_frame
                .duration_since(last_frame)
                .as_secs_f32();

            last_frame = current_frame;

            window.poll_events();

            for (_, event) in glfw::flush_messages(&window.events()) {
                match event {
                    glfw::WindowEvent::CursorPos(x, y) => {
                        if first_mouse {
                            last_mouse_x = x;
                            last_mouse_y = y;
                            first_mouse = false;
                        }

                        let x_offset = x - last_mouse_x;
                        let y_offset = last_mouse_y - y;

                        last_mouse_x = x;
                        last_mouse_y = y;

                        scene.camera.yaw +=
                            (x_offset as f32)
                            * scene.camera.sensitivity.to_radians();

                        scene.camera.pitch +=
                            (y_offset as f32)
                            * scene.camera.sensitivity.to_radians();

                        scene.camera.pitch = scene.camera.pitch.clamp(
                            -89.0_f32.to_radians(),
                            89.0_f32.to_radians(),
                        );
                    }
                    _ => {}
                }
            }


            let direction = scene.camera.direction();
            let right = direction.cross(Vec3::Y).normalize();

            let velocity = scene.camera.speed * delta_time;
    
            if window.is_key_pressed(Key::Escape) {
                window.set_should_close(true);
            }

            if window.is_key_pressed(Key::W){
                scene.camera.position += direction * velocity;
            }

            if window.is_key_pressed(Key::S){
                scene.camera.position -= direction * velocity;
            }

            if window.is_key_pressed(Key::A){
                scene.camera.position -= right * velocity;
            }

            if window.is_key_pressed(Key::D){
                scene.camera.position += right * velocity;
            }
            if window.is_key_pressed(Key::Space){
                scene.camera.position.y += velocity;
            }

            if window.is_key_pressed(Key::LeftShift) {
                scene.camera.position.y -= velocity;
            }
            renderer.begin_frame();
            renderer.render(&scene);

            window.swap_buffers();

        }
        renderer.destroy();

    }

}

