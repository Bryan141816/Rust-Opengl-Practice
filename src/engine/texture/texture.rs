#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureHandle(pub u32);

impl TextureHandle {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Texture {
    pub fn load(path: &str) -> Result<Self, String> {
        let image = image::open(path)
            .map_err(|e|format!("Failed to load texture: {e}"))?
            .flipv()
            .to_rgba8();
        let (width, height) = image.dimensions();
        Ok(Self { width, height, data: image.into_raw() })
    }
}