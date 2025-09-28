use rand::seq::IndexedRandom;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum BodyPart {
    LeftHand,
    RightHand,
    LeftFoot,
    RightFoot,
}

impl BodyPart {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let body_parts = [
            Self::LeftHand,
            Self::RightHand,
            Self::LeftFoot,
            Self::RightFoot,
        ];

        *body_parts.choose(&mut rng).unwrap()
    }
}

impl fmt::Display for BodyPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BodyPart::LeftHand => "🫲 (Left Hand)",
                BodyPart::RightHand => "🫱 (Right Hand)",
                BodyPart::LeftFoot => "🦶 (Left Foot)",
                BodyPart::RightFoot => "🦶 (Right Foot)",
            }
        )
    }
}
