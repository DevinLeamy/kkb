mod error;
mod images;
mod openai;
mod parser;
mod utils;

use clap::Parser;

mod prelude {
    pub use crate::error::*;
    pub use crate::images::*;
    pub use crate::openai::*;
    pub use crate::parser::*;
    pub use crate::utils::*;
    pub use serde::*;
    pub use std::path::PathBuf;
}

use prelude::*;

const DEFAULT_PROMPT: &'static str = "A beautiful desktop image. Ultra realistic.";

fn main() -> Result<()> {
    let args = Arguments::parse();
    let output_path = if let Some(path) = args.output {
        assert!(path.ends_with(".png"));
        path_as_absolute_path(&path)
    } else {
        generate_image_path()
    };
    println!("🌀 Generating...");
    let image_gen = OpenAIImageGen::new()?;
    let prompt = args.prompt.unwrap_or(DEFAULT_PROMPT.into());
    let image = image_gen.create_image(ImageRequest {
        description: prompt.clone(),
        width: 1792,
        height: 1024,
    })?;

    image.save(&output_path)?;
    println!("🎆 Saved image to ${:?}", output_path);
    wallpaper::set_from_path(output_path.to_str().unwrap()).unwrap();
    println!("😎 Set new wallpaper");

    Ok(())
}
