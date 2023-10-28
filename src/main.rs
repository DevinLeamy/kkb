mod error;
mod images;
mod openai;
mod parser;
mod upscale;
mod utils;

use clap::Parser;

mod prelude {
    pub use crate::error::*;
    pub use crate::images::*;
    pub use crate::openai::*;
    pub use crate::parser::*;
    pub use crate::upscale::*;
    pub use crate::utils::*;
    pub use serde::*;
    pub use std::path::PathBuf;
}

use prelude::*;

const IMAGE_SCALING_FACTOR: i32 = 4;
const ASSETS_PATH: &'static str = "/Users/Devin/Desktop/Github/DevinLeamy/kkb/assets";

fn main() -> Result<()> {
    let args = Arguments::parse();
    let output_path = if let Some(path) = args.output {
        assert!(path.ends_with(".png"));
        path_as_absolute_path(&path)
    } else {
        generate_image_path(ASSETS_PATH)
    };
    let image_gen = OpenAIImageGen::new()?;
    let prompt = args.prompt.unwrap_or("Tranquility".to_string());
    let image = image_gen.create_image(ImageRequest {
        description: prompt.clone(),
        width: 1024,
        height: 1024,
    })?;

    let mut upscaler = OpenCVUpscaler::new(IMAGE_SCALING_FACTOR)?;
    let image = upscaler.upscale(image)?;

    image.save(&output_path)?;
    wallpaper::set_from_path(output_path.to_str().unwrap()).unwrap();

    Ok(())
}
