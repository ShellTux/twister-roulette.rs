use clap::Parser;
use twister_roulette::{body_part::BodyPart, color::Color};

#[derive(Debug, Parser)]
struct Cli {
    /// Number of rolls
    #[clap(default_value_t = 1)]
    rolls: usize,
}

fn main() {
    let cli = Cli::parse();

    for _ in 1..=cli.rolls {
        let color = Color::random();
        let body_part = BodyPart::random();

        println!("{} on {}", body_part, color);
    }
}
