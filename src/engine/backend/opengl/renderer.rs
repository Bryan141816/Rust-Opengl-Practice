use std::collections::HashMap;

use glow::HasContext;
use super::light_uniforms::LightUniforms;
use crate::engine::backend::renderer::Renderer;
use crate::engine::{
    scene::resources::{Mesh, MeshHandle}, scene::Scene, shader::{ShaderId, ShaderSource}, texture::{Texture, TextureHandle}
};


use super::{
    mesh::OpenGLMesh,
    shader::OpenGLShader,
    texture::OpenGLTexture,
};
use glam::camera::{rh::proj::opengl};
pub struct OpenGLRendererBackend {
    gl: glow::Context,

    shaders: HashMap<ShaderId, OpenGLShader>,
    meshes: HashMap<MeshHandle, OpenGLMesh>,
    textures: HashMap<TextureHandle, OpenGLTexture>,

    next_mesh_id: u32,
    next_texture_id: u32,
    next_shader_id: u32,
}
impl OpenGLRendererBackend {
    pub unsafe fn new(gl: glow::Context) -> Self {
        unsafe {
            gl.enable(glow::DEPTH_TEST);
        }

        Self {
            gl,

            shaders: HashMap::new(),
            meshes: HashMap::new(),
            textures: HashMap::new(),

            next_mesh_id: 0,
            next_texture_id: 0,
            next_shader_id: 0,
        }
    }
    pub unsafe fn _upload_shader(
        &mut self,
        source: &ShaderSource,
    ) -> ShaderId {
        unsafe {
            let shader = OpenGLShader::new(&self.gl, source).expect("Failed to load shader");
            let handle = ShaderId::new(self.next_shader_id);
            self.next_shader_id += 1;
            self.shaders.insert(handle, shader);
            handle

        }
    }
    pub unsafe fn _upload_mesh(
        &mut self,
        mesh: &Mesh,
    ) -> MeshHandle {
        unsafe {
            let gpu_mesh = OpenGLMesh::new(&self.gl, mesh).expect("Failed to generate mesh");

            let handle = MeshHandle::new(self.next_mesh_id);

            self.next_mesh_id += 1;

            self.meshes.insert(handle, gpu_mesh);

            handle
        }
    }
    pub unsafe fn _upload_texture(
        &mut self,
        texture: &Texture,
    ) -> TextureHandle {
        unsafe {
            let gpu_texture = OpenGLTexture::new(&self.gl, texture);

            let handle = TextureHandle::new(self.next_texture_id);

            self.next_texture_id += 1;

            self.textures.insert(handle, gpu_texture);

            handle
        }
    }

    unsafe fn _begin_frame(&self) {
        unsafe {
            self.gl.clear_color(0.1, 0.1, 0.1, 1.0);

            self.gl.clear(
                glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT,
            );
        }
    }

    unsafe fn _render(&mut self, scene: &Scene) {
        // let camera = &scene.camera;
        // let projection = opengl::perspective(
        //     camera.fov,
        //     camera.aspect_ratio,
        //     camera.near_plane,
        //     camera.far_plane,
        // );
        // let view = camera.view_matrix();
        // for object in &scene.objects {
        //     let shader = self.shaders
        //         .get(&object.material.shader)
        //         .expect("Shader not found");

        //     let mesh = self.meshes
        //         .get(&object.mesh)
        //         .expect("Mesh not found");


        //     unsafe {
        //         shader.bind(&self.gl);
                
        //         shader.set_mat4(&self.gl, "model", &object.transform);
        //         shader.set_mat4(&self.gl, "view", &view);
        //         shader.set_mat4(&self.gl, "projection", &projection);
        //         shader.set_vec3(&self.gl, "viewPos", camera.position);

        //         LightUniforms::upload(&self.gl, shader, &scene.lights.lights);

        //         shader.set_vec3(&self.gl, "materialAmbient", object.material.ambient);
        //         shader.set_vec3(&self.gl, "materialDiffuse", object.material.diffuse);
        //         shader.set_vec3(&self.gl, "materialSpecular", object.material.specular);
        //         shader.set_float(&self.gl, "materialShininess", object.material.shininess);

        //         if let Some(texture) = object.material.diffuse_texture {
        //             shader.set_int(&self.gl, "hasTexture", 1);
        //             let texture = self.textures.get(&texture).expect("Texture not Found");
        //             texture.bind(&self.gl, 0);
        //         } 
        //         mesh.draw(&self.gl);
        //     }
        // }
    }

    pub fn end_frame(&self) {

    }

    unsafe fn _destroy(&mut self) {
        unsafe {
            for shader in self.shaders.values() {
                shader.destroy(&self.gl);
            }

            for mesh in self.meshes.values() {
                mesh.destroy(&self.gl);
            }

            for texture in self.textures.values() {
                texture.destroy(&self.gl);
            }

            self.shaders.clear();
            self.meshes.clear();
            self.textures.clear();
        }
    }

}

impl Renderer for OpenGLRendererBackend {
    fn begin_frame(&mut self) {
        unsafe {
            self._begin_frame();
        }
    }
    fn render(&mut self, scene: &Scene) {
        unsafe  {
            self._render(scene);
        }
    }
    fn destroy(&mut self) {
        unsafe  {
            self._destroy();
        }
    }
    fn upload_mesh(&mut self, mesh: &Mesh) -> MeshHandle {
        unsafe {
            self._upload_mesh(mesh)
        }
    }

    fn upload_texture(&mut self, texture: &Texture) -> TextureHandle {
        unsafe {
            self._upload_texture(texture)
        }
    }

    fn upload_shader(&mut self,  source: &ShaderSource) -> ShaderId{
        unsafe {
            self._upload_shader(source)
        }
    }
}