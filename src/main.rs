extern crate glfw;

use std::path::Path;
use std::fs;

use glfw::{Action, Context, Key::{self}};
use glow::HasContext;
use glam::{Vec3, Mat4};
use glam::camera::rh;
use glam::camera::rh::proj::opengl;
const WINDOW_TITLE: &str = "My App";

struct Mesh {
    vao: glow::NativeVertexArray,
    vbo: glow::NativeBuffer,
    vertex_count: i32,
}


fn create_cube_vertices() -> Vec<f32> {
    let mut vertices = Vec::new();

    let faces = [
        // Front
        (
            Vec3::Z,
            [
            Vec3::new(-0.5, -0.5,  0.5),
            Vec3::new( 0.5, -0.5,  0.5),
            Vec3::new( 0.5,  0.5,  0.5),
            Vec3::new(-0.5,  0.5,  0.5),
            ],
        ),

        // Back
        (
            -Vec3::Z,
            [
            Vec3::new( 0.5, -0.5, -0.5),
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(-0.5,  0.5, -0.5),
            Vec3::new( 0.5,  0.5, -0.5),
            ],
        ),

        // Left
        (
            -Vec3::X,
            [
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(-0.5, -0.5,  0.5),
            Vec3::new(-0.5,  0.5,  0.5),
            Vec3::new(-0.5,  0.5, -0.5),
            ],
        ),

        // Right
        (
            Vec3::X,
            [
            Vec3::new( 0.5, -0.5,  0.5),
            Vec3::new( 0.5, -0.5, -0.5),
            Vec3::new( 0.5,  0.5, -0.5),
            Vec3::new( 0.5,  0.5,  0.5),
            ],
        ),

        // Top
        (
            Vec3::Y,
            [
            Vec3::new(-0.5,  0.5,  0.5),
            Vec3::new( 0.5,  0.5,  0.5),
            Vec3::new( 0.5,  0.5, -0.5),
            Vec3::new(-0.5,  0.5, -0.5),
            ],
        ),

        // Bottom
        (
            -Vec3::Y,
            [
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new( 0.5, -0.5, -0.5),
            Vec3::new( 0.5, -0.5,  0.5),
            Vec3::new(-0.5, -0.5,  0.5),
            ],
        ),
        ];

        for (normal, corners) in faces {
            let indices = [0, 1, 2, 0, 2, 3];

            for index in indices {
                let position = corners[index];

                vertices.extend_from_slice(&[
                    position.x,
                    position.y,
                    position.z,

                    normal.x,
                    normal.y,
                    normal.z,
                ]);
            }
        }

        vertices
}

fn load_shader_source(path: &str) -> String {
    let shader_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(path);

    fs::read_to_string(shader_path).expect("Failed to load file")
}

unsafe fn create_mesh(
    gl: &glow::Context,
    vertices: &[f32],
) -> Mesh {
    unsafe {
        let vao = gl.create_vertex_array().unwrap();
        let vbo = gl.create_buffer().unwrap();

        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

        let vertext_data = std::slice::from_raw_parts(
            vertices.as_ptr() as *const u8,
            std::mem::size_of_val(vertices),
        );

        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER, 
            vertext_data, 
            glow::STATIC_DRAW,
        );

        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 6*4, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 6*4, 3*4);
        gl.enable_vertex_attrib_array(1);

        gl.bind_vertex_array(None);
        gl.bind_buffer(glow::ARRAY_BUFFER, None);

        Mesh {
            vao,
            vbo,
            vertex_count: (vertices.len() / 3) as i32,
        }
    }
}


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


        let cube_vertices = create_cube_vertices();
        let square =   create_mesh(&gl, &cube_vertices);

        let vertex_shader_source =
            load_shader_source("shaders/basic.vert");

        let fragment_shader_source =
            load_shader_source("shaders/basic.frag");
        let vertext_shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();

        gl.shader_source(vertext_shader,&vertex_shader_source);

        gl.compile_shader(vertext_shader);

        assert!(
            gl.get_shader_compile_status(vertext_shader),
            "{}", 
            &gl.get_shader_info_log(vertext_shader)
        );

        let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();

        gl.shader_source(fragment_shader, &fragment_shader_source);

        gl.compile_shader(fragment_shader);

        assert!(
            gl.get_shader_compile_status(fragment_shader),
            "{}",
            &gl.get_shader_info_log(fragment_shader)
        );

        let program = gl.create_program().unwrap();

        gl.attach_shader(program, vertext_shader);
        gl.attach_shader(program, fragment_shader);
        gl.link_program(program);

        assert!(
            gl.get_program_link_status(program),
            "{}",
            gl.get_program_info_log(program)
        );

        gl.delete_shader(vertext_shader);
        gl.delete_shader(fragment_shader);

        let model_location = gl.get_uniform_location(program, "model");
        let view_location = gl.get_uniform_location(program, "view");
        let projection_location = gl.get_uniform_location(program, "projection");

        let object_color_location =
            gl.get_uniform_location(program, "objectColor");

        let light_color_location =
            gl.get_uniform_location(program, "lightColor");

        let light_dir_location =
            gl.get_uniform_location(program, "lightDir");

        let model = Mat4::IDENTITY;
        let view = rh::view::look_at_mat4(
            Vec3::new(0.0,0.0,3.0),
            Vec3::ZERO,
            Vec3::Y,
        );

        let projection = opengl::perspective(
            45.0_f32.to_radians(),
            800.0 / 600.0,
            0.1,
            100.0,
        );

        let mut camera_yaw = 0.0_f32;
        let mut camera_pitch = 0.0_f32;

        let camera_rotation_speed = 2.0_f32.to_radians();
        let camera_distance = 3.0_f32;
        while !window.should_close() {
            gl.clear_color(0.1, 0.1, 0.1, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            gl.use_program(Some(program));


            gl.uniform_matrix_4_f32_slice(
                model_location.as_ref(),
                false,
                &model.to_cols_array(),
            );
            let camera_position = Vec3::new(
                camera_distance * camera_yaw.sin() * camera_pitch.cos(),
                camera_distance * camera_pitch.sin(),
                camera_distance * camera_yaw.cos() * camera_pitch.cos(),
            );

            let view = rh::view::look_at_mat4(
                camera_position,
                Vec3::ZERO,
                Vec3::Y,
            );
            gl.uniform_matrix_4_f32_slice(
                view_location.as_ref(), 
                false, 
                &view.to_cols_array()
            );

            gl.uniform_matrix_4_f32_slice(
                projection_location.as_ref(), 
                false, 
                &projection.to_cols_array()
            );

            // Object color (orange/red)
            gl.uniform_3_f32(
                object_color_location.as_ref(),
                1.0, 0.3, 0.2,
            );

            // Light color (white)
            gl.uniform_3_f32(
                light_color_location.as_ref(),
                1.0, 1.0, 1.0,
            );

            // Direction toward the light source
            gl.uniform_3_f32(
                light_dir_location.as_ref(),
                -0.5, 1.0, 0.3,
            );
            gl.bind_vertex_array(Some(square.vao));
            gl.draw_arrays(glow::TRIANGLES, 0, square.vertex_count);

            window.swap_buffers();

            glfw.poll_events();
            for (_,event) in glfw::flush_messages(&event) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(
                        Key::Escape, _, Action::Press, _
                    ) => {
                        window.set_should_close(true);
                    }

                    glfw::WindowEvent::Key(
                        Key::Left, _, Action::Press | Action::Repeat, _
                    ) => {
                        camera_yaw -= camera_rotation_speed;
                    }

                    glfw::WindowEvent::Key(
                        Key::Right, _, Action::Press | Action::Repeat, _
                    ) => {
                        camera_yaw += camera_rotation_speed;
                    }

                    glfw::WindowEvent::Key(
                        Key::Up, _, Action::Press | Action::Repeat, _
                    ) => {
                        camera_pitch += camera_rotation_speed;
                    }

                    glfw::WindowEvent::Key(
                        Key::Down, _, Action::Press | Action::Repeat, _
                    ) => {
                        camera_pitch -= camera_rotation_speed;
                    }

                    _ => (),
                }

                camera_pitch = camera_pitch.clamp(
                    -89.0_f32.to_radians(),
                    89.0_f32.to_radians(),
                );       
            }
        }
        gl.delete_program(program);
        gl.delete_buffer(square.vbo);
        gl.delete_vertex_array(square.vao);

    }

}
