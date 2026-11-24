use glam::{Vec2, Vec3};

use crate::engine::scene::resources::{Material, Mesh, Vertex};

pub enum LoadObjResults {
    MeshOnly(Mesh),
    WithMaterial {
        mesh: Mesh,
        material: Material,
    },
}

pub struct LoadObj;

impl LoadObj {
    pub fn load_obj(path: &str) -> Result<Mesh, Box<dyn std::error::Error>> {
        let (models, materials) = tobj::load_obj(
            path, 
            &tobj::GPU_LOAD_OPTIONS,
        )?;
        let materials = materials?;
        let model = &models[0];
        // if let Some(material_id) = model.mesh.material_id {
        //     let material = &materials[&material_id];

        // }
        let positions = &model.mesh.positions;
        let normals = &model.mesh.normals;
        let texcoords = &model.mesh.texcoords;

        let vertices: Vec<Vertex> = positions
            .chunks_exact(3)
            .enumerate()
            .map(|(i, pos)| {
                let normal = if normals.len() >= (i + 1) * 3 {
                    Vec3::new(
                        normals[i * 3],
                        normals[i * 3 + 1],
                        normals[i * 3 + 2],
                    )
                } else {
                    Vec3::ZERO
                };

                let tex_coords = if texcoords.len() >= (i + 1) * 2 {
                    Vec2::new(
                        texcoords[i * 2],
                        texcoords[i * 2 + 1],
                    )
                } else {
                    Vec2::ZERO
                };

                Vertex {
                    position: Vec3::new(pos[0], pos[1], pos[2]),
                    normal,
                    tex_coords,
                }
            }).collect();
        let mesh = Mesh::new(vertices, model.mesh.indices.clone());
        
        Ok(mesh)
    }
}