use crate::prelude::*;

const OPENAI_IMAGE_GEN_URL: &'static str = "https://api.openai.com/v1/images/generations";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIImages {
    pub created: u64,
    pub data: Option<Vec<OpenAIImageData>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIImageData {
    pub b64_json: String,
}

pub struct OpenAIImageGen {
    key: String,
}

impl OpenAIImageGen {
    pub fn new() -> Result<Self> {
        let key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| KKBError::Undefined("Missing OPENAI_API_KEY"))?;

        Ok(Self { key })
    }
}

impl ImageGenerator for OpenAIImageGen {
    fn create_image(&self, request: ImageRequest) -> Result<Image> {
        let response = ureq::post(OPENAI_IMAGE_GEN_URL)
            .set("Authorization", &format!("Bearer {}", self.key))
            .send_json(ureq::json!({
                "prompt": request.description,
                "response_format": "b64_json",
                "size": "1024x1024"
            }))
            .unwrap()
            .into_string()
            .unwrap();

        // Error handling, yeah, yeah...
        let images: OpenAIImages = serde_json::from_str(&response).unwrap();
        let image = &images.data.unwrap()[0];
        let base64 = image.b64_json.clone();

        Ok(Image::from_base64(base64))
    }
}
