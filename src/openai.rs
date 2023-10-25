use crate::prelude::*;
use openai_api_rust::*;

pub struct OpenAIImageGen {
    openai: OpenAI,
}

impl OpenAIImageGen {
    pub fn new() -> Self {
        let auth = Auth::from_env().unwrap();
        let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
        Self { openai }
    }
}

impl ImageGenerator for OpenAI {
    fn create_image(&self, request: ImageRequest) -> Result<ImageResponse> {
        todo!()
    }
}
