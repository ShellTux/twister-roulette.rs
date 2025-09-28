use colored::Colorize;
use enum_derive::Random;
use rand::seq::IndexedRandom;
use std::fmt;

#[derive(Debug, Clone, Copy, Random)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
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
