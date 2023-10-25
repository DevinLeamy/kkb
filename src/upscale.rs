use crate::prelude::*;
use opencv::dnn_superres::DnnSuperResImpl;

pub trait ImageUpscaler {}

pub struct OpenCVUpscaler {
    upscaler: DnnSuperResImpl,
}

impl OpenCVUpscaler {
    pub fn new(scale: i32) -> Result<Self> {
        let upscaler = DnnSuperResImpl::new("edsr", scale).map_err(|_| KKBError::Undefined(""))?;

        Ok(Self { upscaler })
    }
}

impl OpenCVUpscaler {}

impl ImageUpscaler for OpenCVUpscaler {}
