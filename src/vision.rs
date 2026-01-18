use serde::Serialize;
pub mod card;
pub mod token;
pub mod unified;

use card::{CardRecognizer, DetectedCard};
use token::{TokenRecognizer, DetectedStack};
use unified::{UnifiedRecognizer, DetectionClass};

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
    unified_recognizer: UnifiedRecognizer,
    card_recognizer: CardRecognizer,
    token_recognizer: TokenRecognizer,
}

impl VisionPipeline {
    pub fn new() -> Self {
        Self {
            unified_recognizer: UnifiedRecognizer::new(),
            card_recognizer: CardRecognizer::new(),
            token_recognizer: TokenRecognizer::new(),
        }
    }

    pub async fn process_frame(&self, frame_data: &[u8]) -> Result<RecognitionResult, String> {
        // 1. Decode image
        let img = image::load_from_memory(frame_data)
            .map_err(|e| format!("Failed to decode image: {}", e))?;

        // 2. Combined YOLO detection for cards and tokens
        let detections = self.unified_recognizer.run_yolo_detection(&img);

        let mut stacks = Vec::new();
        let mut cards = Vec::new();

        // 3. Process detections based on class
        for detection in detections {
            match detection.class {
                DetectionClass::Card => {
                    let detected_card = self.card_recognizer.process_crop(detection.bbox, &detection.crop);
                    cards.push(detected_card);
                }
                DetectionClass::TokenStack => {
                    let detected_stack = self.token_recognizer.process_crop(detection.bbox, &detection.crop);
                    stacks.push(detected_stack);
                }
            }
        }

        Ok(RecognitionResult { stacks, cards })
    }
}
