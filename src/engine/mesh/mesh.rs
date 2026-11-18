use glow::HasContext;

const VERTEX_SIZE: i32 = 8;
const STRIDE: i32 = VERTEX_SIZE * 4;

const POSITION_OFFSET: i32 = 0;
const NORMAL_OFFSET: i32 = 3 * 4;
const TEXTURE_COORDINATES_OFFSET:i32 = 6*4;


pub struct Mesh {
    pub vao: glow::NativeVertexArray,
    pub vbo: glow::NativeBuffer,
    pub ebo: glow::NativeBuffer,


    pub index_count: i32,
}

impl Mesh {
    pub unsafe fn new(gl: &glow::Context, vertices: &[f32], indices: &[u32]) -> Self{
        unsafe {
            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();
            let ebo = gl.create_buffer().unwrap();

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));


            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER, 
                bytemuck::cast_slice(vertices), 
                glow::STATIC_DRAW,
            );

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));

            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER, 
                bytemuck::cast_slice(indices), 
                glow::STATIC_DRAW,
            );

            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, STRIDE, POSITION_OFFSET);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, STRIDE, NORMAL_OFFSET);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, STRIDE, TEXTURE_COORDINATES_OFFSET);
            gl.enable_vertex_attrib_array(2);



            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);


            Self {
                vao,
                vbo,
                ebo,
                index_count: indices.len() as i32
            }
        }
    }
    pub unsafe fn draw(&self, gl: &glow::Context) {
        unsafe  {
            gl.bind_vertex_array(Some(self.vao));

            gl.draw_elements(
                glow::TRIANGLES, 
                self.index_count, 
                glow::UNSIGNED_INT, 
                0
            );

            gl.bind_vertex_array(None);
        }
    }
}
