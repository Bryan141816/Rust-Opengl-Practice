use glow::HasContext;

pub struct Texture {
    pub id: glow::NativeTexture,
}

impl Texture {
   pub unsafe fn new(
        gl: &glow::Context,
        path: &str,
    ) -> Self {
        let image = image::open(path)
            .expect("Failed to load texture")
            .flipv()
            .to_rgba8();

        let (width, height) = image.dimensions();
        unsafe {
            let texture = gl.create_texture().unwrap();

            gl.bind_texture(glow::TEXTURE_2D, Some(texture));

            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(Some(image
                        .as_raw())),
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
                id: texture,
            }
        }
    }
   pub unsafe fn bind(&self, gl:&glow::Context, slot: u32){
       unsafe {
        gl.active_texture(glow::TEXTURE0 + slot);
        gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
       }
   }
}
