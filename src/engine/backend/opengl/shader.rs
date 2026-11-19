use std::{fs,path::Path};

use glam::{Mat4, Vec3};
use glow::HasContext;
use crate::engine::shader::ShaderSource;
pub struct OpenGLShader {
    program: glow::NativeProgram
}

impl OpenGLShader {
        pub unsafe fn new(
        gl: &glow::Context,
        source: &ShaderSource,
    ) -> Result<Self, String> {
        unsafe {
            let vertex_shader =
                Self::compile(gl, glow::VERTEX_SHADER, &source.vertex)?;

            let fragment_shader =
                match Self::compile(gl, glow::FRAGMENT_SHADER, &source.fragment) {
                    Ok(shader) => shader,
                    Err(error) => {
                        gl.delete_shader(vertex_shader);
                        return Err(error);
                    }
                };

            let program = match gl.create_program() {
                Ok(program) => program,
                Err(error) => {
                    gl.delete_shader(vertex_shader);
                    gl.delete_shader(fragment_shader);
                    return Err(error.to_string());
                }
            };

            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);

            gl.link_program(program);

            let linked = gl.get_program_link_status(program);
            let info_log = gl.get_program_info_log(program);

            // Detach the shaders after linking.
            gl.detach_shader(program, vertex_shader);
            gl.detach_shader(program, fragment_shader);

            // Once linked, the program no longer needs the shader objects.
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);

            if !linked {
                gl.delete_program(program);
                return Err(format!("Shader linking failed: {info_log}"));
            }

            Ok(Self { program })
        }
    }


    unsafe fn compile(
        gl: &glow::Context,
        shader_type: u32,
        source: &str,
    ) -> Result<glow::NativeShader, String> {
        unsafe  {
            let shader = gl.create_shader(shader_type)
                .map_err(|error|error.to_string())?;
            
            gl.shader_source(shader, source);
            gl.compile_shader(shader);

            if !gl.get_shader_compile_status(shader){
                let error = gl.get_shader_info_log(shader);
                gl.delete_shader(shader);

                return Err(format!("Shader compiliation failed: {}", error));
            }

            Ok(shader)
        }
    }
    pub unsafe fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }

    pub unsafe fn set_vec3(
        &self,
        gl: &glow::Context,
        name: &str,
        value: Vec3,
    ) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);

            gl.uniform_3_f32(
                location.as_ref(), 
                value.x, 
                value.y, 
                value.z
            );
        }
    }
    pub unsafe fn set_float(
        &self,
        gl: &glow::Context,
        name: &str,
        value: f32,
    ) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_1_f32(location.as_ref(), value);
        }
    }

    pub unsafe fn set_int(
        &self,
        gl: &glow::Context,
        name: &str,
        value: i32,
    ) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_1_i32(location.as_ref(), value);
        }
    }

    pub unsafe fn set_mat4(
        &self,
        gl: &glow::Context,
        name: &str,
        value: &Mat4,
    ) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);

            gl.uniform_matrix_4_f32_slice(
                location.as_ref(),
                false,
                &value.to_cols_array(),
            );
        }
    }

    pub unsafe fn destroy(&self, gl: &glow::Context) {
        unsafe{
            gl.delete_program(self.program);
        }
    }
}
