use crate::vision::BoundingBox;

pub enum DetectionClass {
    Card,
    TokenStack,
}

pub struct Detection {
    pub class: DetectionClass,
    pub bbox: BoundingBox,
    pub crop: image::DynamicImage,
}

pub struct UnifiedRecognizer {
    // Session for the combined model
}

impl UnifiedRecognizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run_yolo_detection(&self, img: &image::DynamicImage) -> Vec<Detection> {
        // In a real implementation, this would call a single YOLO model 
        // that detects both cards and tokens.
        vec![
            Detection {
                class: DetectionClass::Card,
                bbox: BoundingBox { x: 50.0, y: 50.0, width: 60.0, height: 90.0 },
                crop: img.crop_imm(50, 50, 60, 90),
            },
            Detection {
                class: DetectionClass::TokenStack,
                bbox: BoundingBox { x: 100.0, y: 150.0, width: 50.0, height: 80.0 },
                crop: img.crop_imm(100, 150, 50, 80),
            },
        ]
    }
}
