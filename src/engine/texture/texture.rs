#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureHandle(pub u32);
use image::GenericImageView;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Texture {
    pub fn load(path: &str) -> Result<Self, String> {
        let image = image::open(path)
            .map_err(|e| format!("Failed to load texture '{}': {}", path, e))?;

        let (width, height) = image.dimensions();

        let image = image.flipv().to_rgba8();

        Ok(Self {
            width,
            height,
            data: image.into_raw(),
        })
    }
}