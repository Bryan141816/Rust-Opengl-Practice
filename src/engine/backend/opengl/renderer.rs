use std::collections::HashMap;

use glow::HasContext;
use super::light_uniforms::LightUniforms;
use crate::engine::asset::AssetManager;
use crate::engine::backend::renderer::Renderer;
use crate::engine::scene::components::Light;
use crate::engine::scene::resources::MaterialHandle;
use crate::engine::scene::Component;
use crate::engine::scene::traversal::SceneTraversal;
use crate::engine::{
    scene::resources::{Mesh, MeshHandle}, scene::Scene, shader::{ShaderId, ShaderSource}, texture::{Texture, TextureHandle}
};

use super::{
    mesh::OpenGLMesh,
    shader::OpenGLShader,
    texture::OpenGLTexture,
};

use glam::{Mat4, Vec3};

struct PreparedMesh {
    mesh: MeshHandle,
    material: MaterialHandle,
    model: Mat4,
}

struct PreparedCamera {
    pub view: Mat4,
    pub position: Vec3,
    pub projection: Mat4,
}
#[derive(Debug, Clone)]
pub(crate) struct PreparedLight {
    pub(crate) light: Light,
    pub(crate) world_transform: Mat4,
}
#[derive(Default)]
struct RenderData {
    meshes: Vec<PreparedMesh>,
    camera: Option<PreparedCamera>,
    lights: Vec<PreparedLight>,
}
impl RenderData {
    pub fn clear(&mut self) {
        self.meshes.clear();
        self.camera = None;
        self.lights.clear();
    }
}
pub struct OpenGLRendererBackend {
    gl: glow::Context,

    shaders: HashMap<ShaderId, OpenGLShader>,
    meshes: HashMap<MeshHandle, OpenGLMesh>,
    textures: HashMap<TextureHandle, OpenGLTexture>,
    render_data: RenderData,
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
            render_data: RenderData::default()
        }
    }

    pub unsafe fn _upload_shader(
        &mut self,
        id: ShaderId,
        source: &ShaderSource,
    )  {
        unsafe {
            let shader = OpenGLShader::new(&self.gl, source).expect("Failed to load shader");
            self.shaders.insert(id, shader);
        }
    }
    pub unsafe fn _upload_mesh(
        &mut self,
        handle: MeshHandle,
        mesh: &Mesh,
    ) {
        unsafe {
            let gpu_mesh = OpenGLMesh::new(&self.gl, mesh).expect("Failed to generate mesh");
            self.meshes.insert(handle, gpu_mesh);

        }
    }
    pub unsafe fn _upload_texture(
        &mut self,
        handle: TextureHandle,
        texture: &Texture,
    ) {
        unsafe {
            let gpu_texture = OpenGLTexture::new(&self.gl, texture);
            self.textures.insert(handle, gpu_texture);
        }
    }

    fn prepare(&mut self, scene: &Scene) {
        self.render_data.clear();

        let root_id = scene.find_node("Root")
            .expect("Scene root not found")
            .id;

        SceneTraversal::traverse(scene, root_id, &mut |node, world_transform, _depth| {
            for component in &node.components {
                match component {
                    Component::Mesh { instance, .. } => {
                        self.render_data.meshes.push(PreparedMesh {
                            mesh: instance.mesh,
                            material: instance.material,
                            model: world_transform,
                        });
                    }

                    Component::Camera { camera, .. } => {
                        self.render_data.camera = Some(PreparedCamera {
                            view: camera.view_matrix(world_transform),
                            position: world_transform.transform_point3(Vec3::ZERO),
                            projection: camera.projection_matrix(),
                        });
                    }

                    Component::Light { light, .. } => {
                        self.render_data.lights.push(PreparedLight {
                            light: light.clone(),
                            world_transform,
                        });
                    }
                }
            }
        });
    }

    unsafe fn _begin_frame(&self) {
        unsafe {
            self.gl.clear_color(0.1, 0.1, 0.1, 1.0);

            self.gl.clear(
                glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT,
            );
        }
    }

    unsafe fn _render(&mut self, scene: &Scene, assets: &AssetManager) {
        self.prepare(&scene);
        let camera = self.render_data.camera.as_ref().unwrap();

        for object in self.render_data.meshes.iter() {
            let material_handle = object.material;
            let material = assets.get_material(material_handle).unwrap();
            let shader = self.shaders.get(&material.shader).unwrap();
            let mesh = self.meshes.get(&object.mesh).unwrap();
            
            unsafe {
                shader.bind(&self.gl);
                shader.set_mat4(&self.gl, "model", &object.model);
                shader.set_mat4(&self.gl, "view", &camera.view);
                shader.set_mat4(&self.gl, "projection", &camera.projection);
                shader.set_vec3(&self.gl, "viewPos", camera.position);

                LightUniforms::upload(&self.gl, shader, &self.render_data.lights);

                shader.set_vec3(&self.gl, "materialAmbient", material.ambient);
                shader.set_vec3(&self.gl, "materialDiffuse", material.diffuse);
                shader.set_vec3(&self.gl, "materialSpecular", material.specular);
                shader.set_float(&self.gl, "materialShininess", material.shininess);
                match material.diffuse_texture {
                    Some(texture_handle) => {
                        shader.set_int(&self.gl, "hasTexture", 1);

                        let texture = self
                            .textures
                            .get(&texture_handle)
                            .expect("Texture not Found");

                        texture.bind(&self.gl, 0);
                    }

                    None => {
                        shader.set_int(&self.gl, "hasTexture", 0);
                    }
                }
                mesh.draw(&self.gl);
            }

        }

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
    fn render(&mut self, scene: &Scene, assets: &AssetManager) {
        unsafe  {
            self._render(scene, assets);
        }
    }
    fn destroy(&mut self) {
        unsafe  {
            self._destroy();
        }
    }
    fn upload_mesh(&mut self, handle: MeshHandle, mesh: &Mesh) {
        unsafe {
            self._upload_mesh(handle, mesh);
        }
    }

    fn upload_texture(&mut self, handle: TextureHandle, texture: &Texture){
        unsafe {
            self._upload_texture(handle,texture);
        }
    }

    fn upload_shader(&mut self,  id: ShaderId, source: &ShaderSource){
        unsafe {
            self._upload_shader(id, source);
        }
    }
}