use glow::HasContext;
use crate::engine::{backend::opengl::texture, texture::Texture};
pub struct OpenGLTexture {
    pub id: glow::NativeTexture,
}

impl OpenGLTexture {
   pub unsafe fn new(
        gl: &glow::Context,
        texture: &Texture,
    ) -> Self {
        unsafe {
            let id = gl
                .create_texture()
                .expect("Failed to create OpenGL texture");

            gl.bind_texture(glow::TEXTURE_2D, Some(id));

            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                texture.width as i32,
                texture.height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(Some(&texture.data)),
            );

            gl.generate_mipmap(glow::TEXTURE_2D);

            // Texture wrapping
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::REPEAT as i32,
            );

            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::REPEAT as i32,
            );

            // Texture filtering
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR_MIPMAP_LINEAR as i32,
            );

            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );

            gl.bind_texture(glow::TEXTURE_2D, None);

            Self {
                id,
            }
        }
    }
    pub unsafe fn bind(&self, gl:&glow::Context, slot: u32){
        unsafe {
            gl.active_texture(glow::TEXTURE0 + slot);
            gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
        }
    }

    pub unsafe fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_texture(self.id);
        }
    }
}
