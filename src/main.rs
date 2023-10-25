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
    pub use std::path::*;
}

use prelude::*;

const IMAGE_SCALING_FACTOR: i32 = 2;

fn main() {
    let args = Arguments::parse();
    let image_gen = OpenAIImageGen::new();
    let upscaler = OpenCVUpscaler::new(IMAGE_SCALING_FACTOR);
}
