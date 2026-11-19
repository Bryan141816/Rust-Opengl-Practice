use glow::HasContext;

use crate::engine::mesh::Mesh;

pub struct OpenGLMesh {
    pub vao: glow::NativeVertexArray,
    pub vbo: glow::NativeBuffer,
    pub ebo: glow::NativeBuffer,
    pub index_count: i32,
}

impl OpenGLMesh {
    pub unsafe fn new(
        gl: &glow::Context,
        mesh: &Mesh,
    ) -> Result<Self, String> {
        // Flatten the Vertex structs into GPU-friendly float data.
        let mut vertex_data: Vec<f32> = Vec::with_capacity(mesh.vertices.len() * 8);

        for vertex in &mesh.vertices {
            vertex_data.extend_from_slice(&[
                vertex.position.x,
                vertex.position.y,
                vertex.position.z,

                vertex.normal.x,
                vertex.normal.y,
                vertex.normal.z,

                vertex.tex_coords.x,
                vertex.tex_coords.y,
            ]);
        }

        let index_count = mesh.indices.len() as i32;
        unsafe {
            // Create OpenGL objects
            let vao = gl.create_vertex_array()?;
            let vbo = gl.create_buffer()?;
            let ebo = gl.create_buffer()?;
    
            // Bind VAO
            gl.bind_vertex_array(Some(vao));
    
            // Upload vertex data
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                std::slice::from_raw_parts(
                    vertex_data.as_ptr() as *const u8,
                    vertex_data.len() * std::mem::size_of::<f32>(),
                ),
                glow::STATIC_DRAW,
            );
    
            // Upload index data
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                std::slice::from_raw_parts(
                    mesh.indices.as_ptr() as *const u8,
                    mesh.indices.len() * std::mem::size_of::<u32>(),
                ),
                glow::STATIC_DRAW,
            );
    
            let stride = (8 * std::mem::size_of::<f32>()) as i32;
    
            // Position (location = 0)
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                stride,
                0,
            );
    
            // Normal (location = 1)
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(
                1,
                3,
                glow::FLOAT,
                false,
                stride,
                (3 * std::mem::size_of::<f32>()) as i32,
            );
    
            // Texture coordinates (location = 2)
            gl.enable_vertex_attrib_array(2);
            gl.vertex_attrib_pointer_f32(
                2,
                2,
                glow::FLOAT,
                false,
                stride,
                (6 * std::mem::size_of::<f32>()) as i32,
            );
    
            // Unbind VAO
            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
    
            Ok(Self {
                vao,
                vbo,
                ebo,
                index_count,
            })
        }

    }
    pub unsafe fn draw(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));
        
            gl.draw_elements(
                glow::TRIANGLES,
                self.index_count,
                glow::UNSIGNED_INT,
                0,
            );
        
            gl.bind_vertex_array(None);
        }
    }
    pub unsafe fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo);
            gl.delete_buffer(self.ebo);
        }
    }
}