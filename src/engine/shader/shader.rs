use std::{fs, path::Path};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShaderId(pub String);

impl ShaderId {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

pub struct ShaderSource {
    pub vertex: String,
    pub fragment: String,
}

impl ShaderSource {
    pub fn load(
        vertex_path: &str,
        fragment_path: &str,
    ) -> Result<Self, String> {
        Ok(Self {
            vertex: Self::load_file(vertex_path)?,
            fragment: Self::load_file(fragment_path)?,
        })
    }

    fn load_file(path: &str) -> Result<String, String> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(path);
        fs::read_to_string(path)
            .map_err(|error|error.to_string())
    }
}