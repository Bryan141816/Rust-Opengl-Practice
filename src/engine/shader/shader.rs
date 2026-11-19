use std::{fs,path::Path};

use glam::{Mat4, Vec3};
use glow::HasContext;

pub struct Shader {
    program: glow::NativeProgram
}

impl Shader {
    pub unsafe fn new (
        gl: &glow::Context,
        vertex_path: &str,
        fragment_path: &str,
    ) -> Result<Self, String> {
        let vertex_source = Self::load_source(vertex_path)?;
        let fragment_source = Self::load_source(fragment_path)?;

        unsafe{
            let vertex_shader = 
                Self::compile(gl, glow::VERTEX_SHADER, &vertex_source)?;
            let fragment_shader =
                match Self::compile(gl, glow::FRAGMENT_SHADER, &fragment_source) {
                    Ok(shader) => shader,
                    Err(error) => {
                        gl.delete_shader(vertex_shader);
                        return Err(error);
                    }
                };

            let program = gl.create_program()
                .map_err(|error|error.to_string())?;

            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);

            gl.link_program(program);

            let linked = gl.get_program_link_status(program);
            let info_log = gl.get_program_info_log(program);

            gl.detach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);

            if !linked {
                gl.delete_program(program);
                return Err(format!("Shader linking failed: {}", info_log));
            }
            Ok(Self {program})
        }
    }
    fn load_source(path: &str) -> Result<String, String> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);

        fs::read_to_string(path)
            .map_err(|error| error.to_string())
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

    pub unsafe fn delete(self, gl: &glow::Context) {
        unsafe{
            gl.delete_program(self.program);
        }
    }
}
