use std::{
    error::Error,
    io::{Error as IoError, ErrorKind},
    path::Path,
};

use glam::{Mat4, Vec2, Vec3};
use gltf::mesh::Mode;

use crate::engine::scene::resources::{Mesh, Vertex};

use super::{model::Model, model_node::ModelNode};

pub struct ModelLoader;

impl ModelLoader {
    pub fn load(path: impl AsRef<Path>) -> Result<Model, Box<dyn Error>> {
        let (document, buffers, _images) = gltf::import(path)?;

        let mut meshes = Vec::new();

        // Maps a GLTF mesh index to its corresponding engine mesh indices.
        let mut mesh_map: Vec<Vec<usize>> = Vec::new();

        // Extract all mesh primitives.
        for gltf_mesh in document.meshes() {
            let mut primitive_indices = Vec::new();

            for primitive in gltf_mesh.primitives() {
                // This loader currently only supports triangle primitives.
                if primitive.mode() != Mode::Triangles {
                    continue;
                }

                let reader = primitive.reader(|buffer| {
                    Some(buffers[buffer.index()].0.as_slice())
                });

                let positions: Vec<Vec3> = reader
                    .read_positions()
                    .ok_or_else(|| {
                        IoError::new(
                            ErrorKind::InvalidData,
                            "Mesh primitive has no vertex positions",
                        )
                    })?
                    .map(Vec3::from)
                    .collect();

                let indices: Vec<u32> = reader
                    .read_indices()
                    .map(|indices| indices.into_u32().collect())
                    .unwrap_or_else(|| {
                        (0..positions.len() as u32).collect()
                    });

                let normals: Vec<Vec3> = reader
                    .read_normals()
                    .map(|normals| normals.map(Vec3::from).collect())
                    .unwrap_or_else(|| {
                        Self::generate_normals(&positions, &indices)
                    });

                let tex_coords: Vec<Vec2> = reader
                    .read_tex_coords(0)
                    .map(|coords| {
                        coords.into_f32()
                            .map(Vec2::from)
                            .collect()
                    })
                    .unwrap_or_else(|| {
                        vec![Vec2::ZERO; positions.len()]
                    });

                let vertices: Vec<Vertex> = positions
                    .iter()
                    .zip(normals.iter())
                    .zip(tex_coords.iter())
                    .map(|((position, normal), tex_coords)| Vertex {
                        position: *position,
                        normal: *normal,
                        tex_coords: *tex_coords,
                    })
                    .collect();

                let mesh = Mesh::new(vertices, indices);

                // Index of this mesh inside the engine's Model.
                let engine_mesh_index = meshes.len();

                meshes.push(mesh);
                primitive_indices.push(engine_mesh_index);
            }

            mesh_map.push(primitive_indices);
        }

        // Load the selected GLTF scene.
        let scene = document
            .default_scene()
            .or_else(|| document.scenes().next())
            .ok_or_else(|| {
                IoError::new(
                    ErrorKind::InvalidData,
                    "GLTF file contains no scene",
                )
            })?;

        let roots = scene
            .nodes()
            .map(|node| Self::load_node(node, &mesh_map))
            .collect();

        Ok(Model::new(meshes, roots))
    }

    fn load_node(
        node: gltf::Node,
        mesh_map: &[Vec<usize>],
    ) -> ModelNode {
        let transform = Mat4::from_cols_array_2d(
            &node.transform().matrix()
        );

        let meshes = node
            .mesh()
            .map(|mesh| mesh_map[mesh.index()].clone())
            .unwrap_or_default();

        let children = node
            .children()
            .map(|child| Self::load_node(child, mesh_map))
            .collect();

        ModelNode::new(
            node.name().map(String::from),
            transform,
            meshes,
            children,
        )
    }

    fn generate_normals(
        positions: &[Vec3],
        indices: &[u32],
    ) -> Vec<Vec3> {
        let mut normals = vec![Vec3::ZERO; positions.len()];

        // Calculate normals for triangle faces and accumulate
        // them into each vertex.
        for triangle in indices.chunks_exact(3) {
            let i0 = triangle[0] as usize;
            let i1 = triangle[1] as usize;
            let i2 = triangle[2] as usize;

            let v0 = positions[i0];
            let v1 = positions[i1];
            let v2 = positions[i2];

            let normal = (v1 - v0).cross(v2 - v0);

            normals[i0] += normal;
            normals[i1] += normal;
            normals[i2] += normal;
        }

        normals
            .into_iter()
            .map(|normal| {
                if normal.length_squared() > 0.000001 {
                    normal.normalize()
                } else {
                    Vec3::Y
                }
            })
            .collect()
    }
}