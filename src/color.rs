use colored::Colorize;
use rand::seq::IndexedRandom;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let colors = [Color::Red, Color::Green, Color::Blue, Color::Yellow];
        *colors.choose(&mut rng).unwrap()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::Red => "🟥 Red".red(),
                Color::Green => "🟩 Green".green(),
                Color::Blue => "🟦 Blue".blue(),
                Color::Yellow => "🟨 Yellow".yellow(),
            }
        )
    }
}
