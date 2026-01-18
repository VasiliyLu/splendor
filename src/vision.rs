use serde::Serialize;
pub mod card;
pub mod token;

use card::{CardRecognizer, DetectedCard};
use token::{TokenRecognizer, DetectedStack};

#[derive(Debug, Serialize)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize)]
pub struct RecognitionResult {
    pub stacks: Vec<DetectedStack>,
    pub cards: Vec<DetectedCard>,
}

pub struct VisionPipeline {
    card_recognizer: CardRecognizer,
    token_recognizer: TokenRecognizer,
}

impl VisionPipeline {
    pub fn new() -> Self {
        Self {
            card_recognizer: CardRecognizer::new(),
            token_recognizer: TokenRecognizer::new(),
        }
    }

    pub async fn process_frame(&self, frame_data: &[u8]) -> Result<RecognitionResult, String> {
        // 1. Decode image
        let img = image::load_from_memory(frame_data)
            .map_err(|e| format!("Failed to decode image: {}", e))?;

        // 2. Recognize tokens
        let stacks = self.token_recognizer.recognize(&img);

        // 3. Recognize board cards
        let cards = self.card_recognizer.recognize(&img);

        Ok(RecognitionResult { stacks, cards })
    }
}
