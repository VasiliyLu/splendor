use serde::Serialize;
use crate::models::Card;
use crate::vision::BoundingBox;
use std::collections::HashMap;
use crate::models::Gem;

#[derive(Debug, Serialize)]
pub struct DetectedCard {
    pub card: Card,
    pub confidence: f32,
    pub bbox: BoundingBox,
}

pub struct CardRecognizer {
    // In a real implementation, this would hold the model session
}

impl CardRecognizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn recognize(&self, _img: &image::DynamicImage) -> Vec<DetectedCard> {
        // Mock implementation for cards
        let detections = self.run_yolo_detection(_img);
        
        let mut cards = Vec::new();
        for (bbox, _crop) in detections {
            let (card, confidence) = self.run_card_classification(&_crop);
            cards.push(DetectedCard {
                card,
                confidence,
                bbox,
            });
        }
        cards
    }

    pub fn process_crop(&self, bbox: BoundingBox, crop: &image::DynamicImage) -> DetectedCard {
        let (card, confidence) = self.run_card_classification(crop);
        DetectedCard {
            card,
            confidence,
            bbox,
        }
    }

    fn run_yolo_detection(&self, _img: &image::DynamicImage) -> Vec<(BoundingBox, image::DynamicImage)> {
        vec![
            (
                BoundingBox { x: 50.0, y: 50.0, width: 60.0, height: 90.0 },
                _img.crop_imm(50, 50, 60, 90)
            )
        ]
    }

    fn run_card_classification(&self, _crop: &image::DynamicImage) -> (Card, f32) {
        let mut costs = HashMap::new();
        costs.insert(Gem::Diamond, 3);
        (
            Card::new(1, 1, 0, costs, Gem::Sapphire),
            0.95
        )
    }
}
