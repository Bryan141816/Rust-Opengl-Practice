use glam::{Vec2, Vec3};

use crate::engine::scene::resources::{Material, Mesh, Vertex};
use yaop::{convert_to_mesh, parse_obj};
use yaop::mesh::Mesh as YaopMesh;
use yaop::parser::MtlMaterial;
#[derive(Debug)]
pub struct MeshMaterialBundle {
    pub mesh: Mesh,
    pub material: Option<MtlMaterial>,
}
impl MeshMaterialBundle {
    pub fn new(mesh: Mesh, material: Option<MtlMaterial>) -> Self {
        Self {
            mesh: mesh,
            material: material
        }
    }
}
#[derive(Debug)]
pub struct LoadObjResult{
    pub models: Vec<MeshMaterialBundle>
}
pub fn convert_mesh(mesh: &YaopMesh) -> Mesh {
    let vertices: Vec<Vertex> = mesh.vertices
        .iter()
        .map(|vertex| Vertex {
            position: Vec3::from_array(vertex.position),
            normal: Vec3::from_array(vertex.normal),
            tex_coords: Vec2::from_array(vertex.tex_coord),
        })
        .collect();

    let indices: Vec<u32> = mesh.indices
        .iter()
        .map(|&index| index as u32)
        .collect();

    Mesh::new(vertices, indices)
}
pub struct LoadObj;

impl LoadObj {
    pub fn load_obj(path: &str) -> Result<LoadObjResult, Box<dyn std::error::Error>> {
        let mut result = LoadObjResult {models: Vec::new()};
        let obj = parse_obj(path).unwrap();
        let model = convert_to_mesh(obj);
        let material = model.materials;
        let meshes = model.meshes;
        for mesh in meshes {
            let new_mesh = convert_mesh(&mesh);
            if let Some(material_id) = mesh.material_id {
                let yaop_material = &material[material_id];
                let mesh_mat_bundle = MeshMaterialBundle::new(new_mesh, Some(yaop_material.clone()));
                result.models.push(mesh_mat_bundle);
            }else{
                let mesh_mat_bundle = MeshMaterialBundle::new(new_mesh, None);
                result.models.push(mesh_mat_bundle);
            }            
        }
        Ok(result)
    }
}