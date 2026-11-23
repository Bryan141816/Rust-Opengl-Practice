use glam::{Vec3, Vec2};
use crate::engine::scene::resources::{Mesh, Vertex};

#[allow(dead_code)]
pub enum Primitive {
    Cube {
        size: f32,
    },
    Sphere {
        radius: f32,
        sectors: u32,
        stacks: u32,
    },
    Plane {
        width: f32,
        height: f32,
    },
}
impl Primitive {
    pub fn generate(&self) -> Mesh {
        match &self {
            Primitive::Cube { size } => {
                let mut vertices = Vec::new();
                let mut indices = Vec::new();

                let faces = [
                    (
                        Vec3::Z,
                        [
                            Vec3::new(-*size, -*size, *size),
                            Vec3::new(*size, -*size, *size),
                            Vec3::new(*size, *size, *size),
                            Vec3::new(-*size, *size, *size),
                        ],
                    ),
                    (
                        -Vec3::Z,
                        [
                            Vec3::new(*size, -*size, -*size),
                            Vec3::new(-*size, -*size, -*size),
                            Vec3::new(-*size, *size, -*size),
                            Vec3::new(*size, *size, -*size),
                        ],
                    ),
                    (
                        -Vec3::X,
                        [
                            Vec3::new(-*size, -*size, -*size),
                            Vec3::new(-*size, -*size, *size),
                            Vec3::new(-*size, *size, *size),
                            Vec3::new(-*size, *size, -*size),
                        ],
                    ),
                    (
                        Vec3::X,
                        [
                            Vec3::new(*size, -*size, *size),
                            Vec3::new(*size, -*size, -*size),
                            Vec3::new(*size, *size, -*size),
                            Vec3::new(*size, *size, *size),
                        ],
                    ),
                    (
                        Vec3::Y,
                        [
                            Vec3::new(-*size, *size, *size),
                            Vec3::new(*size, *size, *size),
                            Vec3::new(*size, *size, -*size),
                            Vec3::new(-*size, *size, -*size),
                        ],
                    ),
                    (
                        -Vec3::Y,
                        [
                            Vec3::new(-*size, -*size, -*size),
                            Vec3::new(*size, -*size, -*size),
                            Vec3::new(*size, -*size, *size),
                            Vec3::new(-*size, -*size, *size),
                        ],
                    ),
                ];

                let face_uvs = [
                    Vec2::new(0.0, 0.0),
                    Vec2::new(1.0, 0.0),
                    Vec2::new(1.0, 1.0),
                    Vec2::new(0.0, 1.0),
                ];

                for (normal, corners) in faces {
                    let base_index = vertices.len() as u32;

                    for (i, position) in corners.into_iter().enumerate() {
                        vertices.push(Vertex {
                            position,
                            normal,
                            tex_coords: face_uvs[i],
                        });
                    }

                    indices.extend_from_slice(&[
                        base_index,
                        base_index + 1,
                        base_index + 2,

                        base_index,
                        base_index + 2,
                        base_index + 3,
                    ]);
                }

                Mesh::new(vertices, indices)
            },
            Primitive::Sphere { radius, sectors, stacks } => {
                let mut vertices = Vec::new();
                let mut indices = Vec::new();

                for i in 0..=*stacks {
                    let stack_angle = std::f32::consts::PI / 2.0
                        - i as f32 * std::f32::consts::PI / *stacks as f32;

                    let xy = *radius * stack_angle.cos();
                    let z = *radius * stack_angle.sin();

                    for j in 0..=*sectors {
                        let sector_angle = j as f32
                            * 2.0
                            * std::f32::consts::PI
                            / *sectors as f32;

                        let x = xy * sector_angle.cos();
                        let y = xy * sector_angle.sin();

                        // Position
                        let position = Vec3::new(x, y, z);

                        // Normal
                        let normal = position.normalize();

                        // Texture coordinates
                        let u = j as f32 / *sectors as f32;
                        let v = i as f32 / *stacks as f32;

                        let tex_coords = Vec2::new(u, v);

                        vertices.push(Vertex {
                            position,
                            normal,
                            tex_coords,
                        });
                    }
                }

                for i in 0..*stacks {
                    let k1 = i * (*sectors + 1);
                    let k2 = k1 + *sectors + 1;

                    for j in 0..*sectors {
                        if i != 0 {
                            indices.push((k1 + j) as u32);
                            indices.push((k2 + j) as u32);
                            indices.push((k1 + j + 1) as u32);
                        }

                        if i != *stacks - 1 {
                            indices.push((k1 + j + 1) as u32);
                            indices.push((k2 + j) as u32);
                            indices.push((k2 + j + 1) as u32);
                        }
                    }
                }

                Mesh::new(vertices, indices)
            },
            Primitive::Plane { width, height } => {
                let half_width = *width / 2.0;
                let half_height = *height / 2.0;

                let normal = Vec3::Y;

                let vertices = vec![
                    Vertex {
                        position: Vec3::new(-half_width, 0.0, -half_height),
                        normal,
                        tex_coords: Vec2::new(0.0, 0.0),
                    },
                    Vertex {
                        position: Vec3::new(-half_width, 0.0, half_height),
                        normal,
                        tex_coords: Vec2::new(0.0, 1.0),
                    },
                    Vertex {
                        position: Vec3::new(half_width, 0.0, half_height),
                        normal,
                        tex_coords: Vec2::new(1.0, 1.0),
                    },
                    Vertex {
                        position: Vec3::new(half_width, 0.0, -half_height),
                        normal,
                        tex_coords: Vec2::new(1.0, 0.0),
                    },
                ];

                let indices = vec![
                    0, 1, 2,
                    0, 2, 3,
                ];

                Mesh::new(vertices, indices)
            }
        }
    }
}
