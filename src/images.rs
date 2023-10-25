use crate::prelude::*;

pub struct Image {}

impl Image {}

impl Image {
    pub fn save(&self, path: impl Into<PathBuf>) -> Result<()> {
        todo!()
    }
}

pub struct ImageRequest {
    description: String,
    width: u32,
    height: u32,
}

pub struct ImageResponse {}

pub trait ImageGenerator {
    fn create_image(&self, request: ImageRequest) -> Result<ImageResponse>;
}
