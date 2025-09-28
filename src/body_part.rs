use enum_derive::Random;
use rand::seq::IndexedRandom;
use std::fmt;

#[derive(Debug, Clone, Copy, Random)]
pub enum BodyPart {
    LeftHand,
    RightHand,
    LeftFoot,
    RightFoot,
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
