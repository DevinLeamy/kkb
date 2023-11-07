use std::os::raw::c_void;

use crate::prelude::*;
use opencv::{
    core::{Vector, CV_32F},
    dnn_superres::DnnSuperResImpl,
    imgcodecs::{imdecode, imencode, IMREAD_COLOR},
    imgproc, photo,
    prelude::{DnnSuperResImplTrait, Mat},
};

const OPENCV_UPSCALING_MODEL: &'static str = "edsr";
const EDSR_MODEL_PATH: &'static str =
    "/Users/Devin/Desktop/Github/DevinLeamy/kkb/assets/models/EDSR_x4.pb";
const ESPCN_MODEL_PATH: &'static str =
    "/Users/Devin/Desktop/Github/DevinLeamy/kkb/assets/models/ESPCN_x4.pb";

pub trait ImageUpscaler {
    fn upscale(&mut self, image: Image) -> Result<Image>;
}

pub struct OpenCVUpscaler {
    upscaler: DnnSuperResImpl,
    scale: i32,
}

impl OpenCVUpscaler {
    pub fn new(scale: i32) -> Result<Self> {
        let mut upscaler = DnnSuperResImpl::new(OPENCV_UPSCALING_MODEL, scale).unwrap();
        upscaler.read_model(EDSR_MODEL_PATH).unwrap();
        upscaler.set_model("edsr", scale).unwrap();
        // upscaler.read_model(ESPCN_MODEL_PATH).unwrap();
        // upscaler.set_model("espcn", scale).unwrap();

        Ok(Self { upscaler, scale })
    }
}

impl OpenCVUpscaler {}

impl ImageUpscaler for OpenCVUpscaler {
    fn upscale(&mut self, image: Image) -> Result<Image> {
        let byte_vector: Vector<u8> = Vector::from_slice(image.bytes().as_slice());
        let in_img = imdecode(&byte_vector, IMREAD_COLOR).expect("failed to decode image");
        let mut upsampled = Mat::default();
        self.upscaler.upsample(&in_img, &mut upsampled).unwrap();

        let mut denoised = Mat::default();
        photo::fast_nl_means_denoising_colored(&upsampled, &mut denoised, 3.0, 3.0, 7, 21).unwrap();

        let data: &[f32] = &[0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0];
        let kernel = unsafe {
            Mat::new_rows_cols_with_data_def(3, 3, CV_32F, data.as_ptr() as *mut c_void).unwrap()
        };
        let mut sharpened = Mat::default();
        imgproc::filter_2d(
            &denoised,
            &mut sharpened,
            -1,
            &kernel,
            opencv::core::Point::new(-1, -1),
            0.0,
            opencv::core::BORDER_DEFAULT,
        )
        .unwrap();

        let mut out_byte_buffer = Vector::<u8>::new();
        imencode(".png", &sharpened, &mut out_byte_buffer, &Vector::new()).unwrap();
        let base64_out = base64::encode(out_byte_buffer.to_vec());
        Ok(Image::from_base64(base64_out))
    }
}
