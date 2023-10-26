use std::{fs::File, io::Write};

use crate::prelude::*;

pub struct Image {
    base64: String,
}

impl Image {
    pub fn from_base64(base64: String) -> Self {
        Self { base64 }
    }

    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        todo!()
    }
}

impl Image {
    pub fn save(&self, path: impl Into<PathBuf>) -> Result<()> {
        let image_bytes = base64::decode(&self.base64).expect("Failed to decode base64 string");
        let mut file = File::create(path.into()).expect("Failed to create file");
        file.write_all(&image_bytes).expect("Failed save image");

        Ok(())
    }
}

pub struct ImageRequest {
    pub description: String,
    pub width: u32,
    pub height: u32,
}

pub trait ImageGenerator {
    fn create_image(&self, request: ImageRequest) -> Result<Image>;
}
