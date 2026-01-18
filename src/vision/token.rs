use serde::Serialize;
use crate::models::Token;
use crate::vision::BoundingBox;

#[derive(Debug, Serialize)]
pub struct DetectedStack {
    pub token_type: Token,
    pub count: u8,
    pub confidence: f32,
    pub bbox: BoundingBox,
}

pub struct TokenRecognizer {
    // In a real implementation, this would hold the model session
}

impl TokenRecognizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn recognize(&self, _img: &image::DynamicImage) -> Vec<DetectedStack> {
        // Mock implementation of Stage 1 & 2 for tokens
        let detections = self.run_yolo_detection(_img);
        
        let mut stacks = Vec::new();
        for (bbox, _crop) in detections {
            let (token_type, count, confidence) = self.run_stack_regression(&_crop);
            stacks.push(DetectedStack {
                token_type,
                count,
                confidence,
                bbox,
            });
        }
        stacks
    }

    pub fn process_crop(&self, bbox: BoundingBox, crop: &image::DynamicImage) -> DetectedStack {
        let (token_type, count, confidence) = self.run_stack_regression(crop);
        DetectedStack {
            token_type,
            count,
            confidence,
            bbox,
        }
    }

    fn run_yolo_detection(&self, _img: &image::DynamicImage) -> Vec<(BoundingBox, image::DynamicImage)> {
        vec![
            (
                BoundingBox { x: 100.0, y: 150.0, width: 50.0, height: 80.0 },
                _img.crop_imm(100, 150, 50, 80)
            )
        ]
    }

    fn run_stack_regression(&self, _crop: &image::DynamicImage) -> (Token, u8, f32) {
        (Token::Sapphire, 7, 0.98)
    }
}
