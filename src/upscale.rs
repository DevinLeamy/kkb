use crate::prelude::*;
use opencv::dnn_superres::DnnSuperResImpl;

const OPENCV_UPSCALING_MODEL: &'static str = "edsr";

pub trait ImageUpscaler {
    fn upscale(&self, image: Image) -> Result<Image>;
}

pub struct OpenCVUpscaler {
    upscaler: DnnSuperResImpl,
}

impl OpenCVUpscaler {
    pub fn new(scale: i32) -> Result<Self> {
        let upscaler = DnnSuperResImpl::new(OPENCV_UPSCALING_MODEL, scale)
            .map_err(|_| KKBError::Undefined(""))?;

        Ok(Self { upscaler })
    }
}

impl OpenCVUpscaler {}

impl ImageUpscaler for OpenCVUpscaler {
    fn upscale(&self, image: Image) -> Result<Image> {
        todo!()
    }
}
