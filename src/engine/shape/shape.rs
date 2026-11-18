use glam::{Vec3};

pub enum Shape {
    Cube {
        size: f32,
    },
    Sphere {
        radius: f32,
        sectors: u32,
        stacks: u32,
    }
}
impl Shape {
    pub fn generate(&self) -> (Vec<f32>, Vec<u32>) {
        match &self {
            Shape::Cube  { size } => {
                let mut vertices = Vec::new();
                let mut indices = Vec::new();

                let faces = [
                    (
                        Vec3::Z,
                        [
                        Vec3::new(-*size, -*size,  *size),
                        Vec3::new( *size, -*size,  *size),
                        Vec3::new( *size,  *size,  *size),
                        Vec3::new(-*size,  *size,  *size),
                        ],
                    ),

                    (
                        -Vec3::Z,
                        [
                        Vec3::new( *size, -*size, -*size),
                        Vec3::new(-*size, -*size, -*size),
                        Vec3::new(-*size,  *size, -*size),
                        Vec3::new( *size,  *size, -*size),
                        ],
                    ),

                    (
                        -Vec3::X,
                        [
                        Vec3::new(-*size, -*size, -*size),
                        Vec3::new(-*size, -*size,  *size),
                        Vec3::new(-*size,  *size,  *size),
                        Vec3::new(-*size,  *size, -*size),
                        ],
                    ),

                    (
                        Vec3::X,
                        [
                        Vec3::new( *size, -*size,  *size),
                        Vec3::new( *size, -*size, -*size),
                        Vec3::new( *size,  *size, -*size),
                        Vec3::new( *size,  *size,  *size),
                        ],
                    ),

                    (
                        Vec3::Y,
                        [
                        Vec3::new(-*size,  *size,  *size),
                        Vec3::new( *size,  *size,  *size),
                        Vec3::new( *size,  *size, -*size),
                        Vec3::new(-*size,  *size, -*size),
                        ],
                    ),

                    (
                        -Vec3::Y,
                        [
                        Vec3::new(-*size, -*size, -*size),
                        Vec3::new( *size, -*size, -*size),
                        Vec3::new( *size, -*size,  *size),
                        Vec3::new(-*size, -*size,  *size),
                        ],
                    ),
                    ];

                    let face_uvs = [
                        [0.0, 0.0], 
                        [1.0, 0.0], 
                        [1.0, 1.0], 
                        [0.0, 1.0], 
                    ];

                    for (normal, corners) in faces {
                        let base_index = (vertices.len() / 8) as u32;

                        for (i, position) in corners.into_iter().enumerate() {
                            let uv = face_uvs[i];

                            vertices.extend_from_slice(&[
                                position.x,
                                position.y,
                                position.z,

                                normal.x,
                                normal.y,
                                normal.z,

                                uv[0],
                                uv[1],
                            ]);
                        }

                        indices.extend_from_slice(&[
                            base_index, base_index + 1, base_index + 2,
                            base_index, base_index + 2, base_index + 3,
                        ]);
                    }

                    (vertices, indices)

            },
            Shape::Sphere { radius, sectors, stacks } => {
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
                        vertices.push(x);
                        vertices.push(y);
                        vertices.push(z);

                        // Normal
                        let normal = Vec3::new(x, y, z).normalize();

                        vertices.push(normal.x);
                        vertices.push(normal.y);
                        vertices.push(normal.z);

                        let u = j as f32 / *sectors as f32;
                        let v = i as f32 / *stacks as f32;

                        vertices.push(u);
                        vertices.push(v);
                    }
                }

                for i in 0..*stacks {
                    let k1 = i * (*sectors + 1);
                    let k2 = k1 + *sectors + 1;

                    for j in 0..*sectors {
                        if i != 0 {
                            indices.push(k1 + j);
                            indices.push(k2 + j);
                            indices.push(k1 + j + 1);
                        }

                        if i != *stacks - 1 {
                            indices.push(k1 + j + 1);
                            indices.push(k2 + j);
                            indices.push(k2 + j + 1);
                        }
                    }
                }

                (vertices, indices)
            }        }
    }
}
