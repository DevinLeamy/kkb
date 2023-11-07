use std::{
    fs::{self, File},
    io::Write,
};

use crate::prelude::*;

pub struct Image {
    base64: String,
}

impl Image {
    pub fn from_base64(base64: String) -> Self {
        Self { base64 }
    }
}

impl Image {
    pub fn save(&self, path: impl Into<PathBuf>) -> Result<()> {
        let mut file = File::create(path.into()).expect("Failed to create file");
        file.write_all(&self.bytes()).expect("Failed save image");

        Ok(())
    }

    pub fn bytes(&self) -> Vec<u8> {
        base64::decode(&self.base64).expect("Failed to decode base64 string")
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
