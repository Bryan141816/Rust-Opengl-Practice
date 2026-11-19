mod engine;
extern crate glfw;

use std::time::Instant;

use glfw::{Action, Context, Key::{self}};
use glow::HasContext;
use glam::{Vec3, Mat4};
use glam::camera::rh::proj::opengl;

use crate::engine::camera::Camera;
use crate::engine::mesh::Mesh;
use crate::engine::texture::Texture;
use crate::engine::shape::Shape;
use crate::engine::lighting::{Light, DirectionalLight, PointLight};
use crate::engine::lighting::Material;
use crate::engine::lighting::LightManager;
use crate::engine::shader::Shader;

const WINDOW_TITLE: &str = "My App";

fn main() {
    use glfw::fail_on_errors;

    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(
        glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core
        )
    );

    let (mut window, event) = glfw
        .create_window(
            800, 
            600, 
            WINDOW_TITLE, 
            glfw::WindowMode::Windowed
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    let gl = unsafe {
        glow::Context::from_loader_function_cstr(|s| {
            glfw.get_proc_address_raw(s.to_str().unwrap())
                .map_or(
                    std::ptr::null(), 
                    |f| f as *const () as *const std::ffi::c_void
                )
        })
    };

    println!("OpenGL initialized");


    unsafe {
        gl.enable(glow::DEPTH_TEST);


        let (cube_vertices, cube_indeces)= Shape::Sphere { radius: 1.0, sectors: 36, stacks: 18 }.generate();
        let cube_mesh = Mesh::new(&gl, &cube_vertices, &cube_indeces);
        let texture = Texture::new(&gl, "assets/textures/crate.jpg");
        let shader = Shader::new(
            &gl,
            "shaders/basic.vert",
            "shaders/basic.frag",
        ).expect("Failed to create shader");


        let model = Mat4::IDENTITY;

        let projection = opengl::perspective(
            45.0_f32.to_radians(),
            800.0 / 600.0,
            0.1,
            100.0,
        );

        let mut camera = Camera::new(
            Vec3::new(0.0, 0.0, 3.0)
        );  

        let mut light_manager = LightManager::new();

        light_manager.add(Light::Directional(DirectionalLight {
            direction: Vec3::new(-0.2, -1.0, -0.3),
            color: Vec3::ONE,
            intensity: 1.0,
        }));
        light_manager.add(Light::Point(PointLight {
            position: Vec3::new(2.0, 2.0, 2.0),
            color: Vec3::new(1.0, 0.0, 0.0),
            intensity: 2.0,

            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
        }));

        light_manager.add(Light::Point(PointLight {
            position: Vec3::new(-2.0, 1.0, 0.0),
            color: Vec3::new(0.0, 0.0, 1.0),
            intensity: 2.0,

            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
        }));

        let material = Material {
            ambient: Vec3::splat(1.0),
            diffuse: Vec3::ONE,
            specular: Vec3::ONE,
            shininess: 32.0,
        };
        let mut last_frame = Instant::now();
        window.set_cursor_pos_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        let mut last_mouse_x = 400.0;
        let mut last_mouse_y = 300.0;
        let mut first_mouse = true;
        while !window.should_close() {


            let current_frame = Instant::now();

            let delta_time = current_frame
                .duration_since(last_frame)
                .as_secs_f32();

            last_frame = current_frame;

            glfw.poll_events();

            for (_, event) in glfw::flush_messages(&event) {
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

                        camera.yaw +=
                            (x_offset as f32)
                            * camera.sensitivity.to_radians();

                        camera.pitch +=
                            (y_offset as f32)
                            * camera.sensitivity.to_radians();

                        camera.pitch = camera.pitch.clamp(
                            -89.0_f32.to_radians(),
                            89.0_f32.to_radians(),
                        );
                    }

                    glfw::WindowEvent::Key(
                        Key::Escape,
                        _,
                        Action::Press,
                        _
                    ) => {
                        window.set_should_close(true);
                    }

                    _ => {}
                }
            }


            let direction = camera.direction();
            let right = direction.cross(Vec3::Y).normalize();

            let velocity = camera.speed * delta_time;

            if window.get_key(Key::W) == Action::Press {
                camera.position += direction * velocity;
            }

            if window.get_key(Key::S) == Action::Press {
                camera.position -= direction * velocity;
            }

            if window.get_key(Key::A) == Action::Press {
                camera.position -= right * velocity;
            }

            if window.get_key(Key::D) == Action::Press {
                camera.position += right * velocity;
            }
            if window.get_key(Key::Space) == Action::Press {
                camera.position.y += velocity;
            }

            if window.get_key(Key::LeftShift) == Action::Press {
                camera.position.y -= velocity;
            }
            gl.clear_color(0.1, 0.1, 0.1, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            shader.bind(&gl);
            // Camera matrix
            let view = camera.view_matrix();

            shader.set_mat4(&gl, "model", &model);
            shader.set_mat4(&gl, "view", &view);
            shader.set_mat4(&gl, "projection", &projection);
            shader.set_vec3(&gl, "viewPos", camera.position);

            let mut directional_count = 0;
            let mut point_count = 0;
            let mut spot_count = 0;

            for light in &light_manager.lights {
                match light {
                    Light::Directional(light) => {
                        let prefix = format!("directionalLights[{}]", directional_count);

                        shader.set_vec3(
                            &gl,
                            &format!("{}.direction", prefix),
                            light.direction,
                        );

                        shader.set_vec3(
                            &gl,
                            &format!("{}.color", prefix),
                            light.color,
                        );

                        shader.set_float(
                            &gl,
                            &format!("{}.intensity", prefix),
                            light.intensity,
                        );

                        directional_count += 1;
                    }

                    Light::Point(light) => {
                        let prefix = format!("pointLights[{}]", point_count);

                        shader.set_vec3(
                            &gl,
                            &format!("{}.position", prefix),
                            light.position,
                        );

                        shader.set_vec3(
                            &gl,
                            &format!("{}.color", prefix),
                            light.color,
                        );

                        shader.set_float(
                            &gl,
                            &format!("{}.intensity", prefix),
                            light.intensity,
                        );

                        shader.set_float(
                            &gl,
                            &format!("{}.constant", prefix),
                            light.constant,
                        );

                        shader.set_float(
                            &gl,
                            &format!("{}.linear", prefix),
                            light.linear,
                        );

                        shader.set_float(
                            &gl, 
                            &format!("{}.quadratic", prefix),
                            light.quadratic,
                        );

                        point_count += 1;
                    }

                    Light::Spot(light) => {
                        let prefix = format!("spotLights[{}]", spot_count);

                        shader.set_vec3(
                            &gl,
                            &format!("{}.position", prefix),
                            light.position,
                        );

                        shader.set_vec3(
                            &gl,
                            &format!("{}.direction", prefix),
                            light.direction,
                        );

                        shader.set_vec3(
                            &gl,
                            &format!("{}.color", prefix),
                            light.color,
                        );

                        shader.set_float(
                            &gl,
                            &format!("{}.intensity", prefix),
                            light.intensity,
                        );

                        shader.set_float(
                            &gl, 
                            &format!("{}.cutOff", prefix),
                            light.cut_off,
                        );

                        shader.set_float(
                            &gl,    
                            &format!("{}.outerCutOff", prefix),
                            light.outer_cut_off,
                        );

                        spot_count += 1;
                    }
                }
            }
            shader.set_int(&gl, "directionalLightCount", directional_count as i32);
            shader.set_int(&gl, "pointLightCount", point_count as i32);
            shader.set_int(&gl, "spotLightCount", spot_count as i32);
            // Material
            shader.set_vec3(&gl, "materialAmbient", material.ambient);
            shader.set_vec3(&gl, "materialDiffuse", material.diffuse);
            shader.set_vec3(&gl, "materialSpecular", material.specular);
            shader.set_float(&gl, "materialShininess", material.shininess);

            // Texture
            shader.set_int(&gl, "hasTexture", 0);
            //texture.bind(&gl, 0);

            shader.set_int(&gl, "texture1", 0);

            cube_mesh.draw(&gl);

            window.swap_buffers();

        }
        shader.delete(&gl);
        gl.delete_buffer(cube_mesh.vbo);
        gl.delete_vertex_array(cube_mesh.vao);

    }

}

