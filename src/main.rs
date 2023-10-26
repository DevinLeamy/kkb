mod error;
mod images;
mod openai;
mod parser;
mod upscale;

use clap::Parser;
use parser::Arguments;
use upscale::*;

mod prelude {
    pub use crate::error::*;
    pub use crate::images::*;
    pub use crate::openai::*;
    pub use crate::parser::*;
    pub use crate::upscale::*;
    pub use serde::*;
    pub use std::path::*;
}

use prelude::*;

const IMAGE_SCALING_FACTOR: i32 = 4;
const ASSETS_PATH: &'static str = "/Users/Devin/Desktop/Github/DevinLeamy/kkb/assets";

fn main() -> Result<()> {
    let args = Arguments::parse();
    // let image_gen = OpenAIImageGen::new()?;
    // let prompt = args.prompt.unwrap_or("Pure beauty".to_string());
    // let image = image_gen.create_image(ImageRequest {
    //     description: prompt.clone(),
    //     width: 1024,
    //     height: 1024,
    // })?;
    // let path = &format!("{ASSETS_PATH}/image-{prompt}.png");
    // image.save(path)?;

    let path = &format!("{ASSETS_PATH}/cloud_city.png");
    let image = Image::from_path(path);
    let mut upscaler = OpenCVUpscaler::new(IMAGE_SCALING_FACTOR)?;
    let scaled_image = upscaler.upscale(image)?;
    scaled_image.save(&format!("{ASSETS_PATH}/image-1-scaled.png"))?;

    Ok(())
}
