use glow::HasContext;

pub struct Mesh {
    pub vao: glow::NativeVertexArray,
    pub vbo: glow::NativeBuffer,
    pub vertex_count: i32,
}

impl Mesh {
    pub unsafe fn new(gl: &glow::Context, vertices: &[f32]) -> Self{
        unsafe {
            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            let vertex_data = std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8, 
                std::mem::size_of_val(vertices),
            );

            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER, 
                vertex_data, 
                glow::STATIC_DRAW
            );

            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 6*4, 0);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 6*4, 3*4);
            gl.enable_vertex_attrib_array(1);

            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);


            Self {
                vao,
                vbo,
                vertex_count: (vertices.len()/6) as i32,
            }
        }
    }
}
