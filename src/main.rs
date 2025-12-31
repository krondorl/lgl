// Copyright 2025 Adam Burucs. Licensed under custom Source Available License

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use rand::prelude::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

const VERSION_INFO: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " — ",
    env!("CARGO_PKG_DESCRIPTION"),
    "\nRNG: PCG64 (high-quality pseudorandom number generator)"
);

/// Generate Mega Millions lottery numbers
#[derive(Parser)]
#[command(version = VERSION_INFO)]
#[command(
    long_about = "This tool generates Mega Millions lottery numbers, which are popular in USA. The random number generator used is PCG64, which is a high-quality, fast, and well-documented pseudorandom number generator.",
    after_help = "Examples:\n  lgl 5              Generate 5 draws\n  lgl 10 draws.txt   Generate 10 draws and save to file"
)]
struct Cli {
    /// The number of draws to generate (default: 1)
    #[arg(default_value_t = 1)]
    count: u8,
    /// Optional path to save the numbers
    path: Option<std::path::PathBuf>,
}

#[derive(Debug)]
struct Draw {
    white_balls: [u8; 5],
    mega_ball: u8,
}

impl std::fmt::Display for Draw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} | Mega Ball: {}", self.white_balls, self.mega_ball)
    }
}

fn generate_draw(rng: &mut Pcg64) -> Result<Draw> {
    let mut white_balls: Vec<u8> = (1..=70).collect();
    white_balls.shuffle(rng);

    let final_numbers: [u8; 5] = white_balls[0..5]
        .try_into()
        .context("Failed to convert slice to array")?;

    let mut sorted_numbers = final_numbers;
    sorted_numbers.sort_unstable();

    Ok(Draw {
        white_balls: sorted_numbers,
        mega_ball: rng.random_range(1..=25),
    })
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let cmd = Cli::command();
    let about = cmd.get_about().map(|s| s.to_string()).unwrap_or_default();
    println!(
        "{} {} — {}",
        cmd.get_name(),
        cmd.get_version().unwrap_or(""),
        about
    );
    println!();

    let mut rng = Pcg64::from_os_rng();
    let mut draws = Vec::new();

    for _ in 0..args.count {
        let draw = generate_draw(&mut rng)?;
        draws.push(draw);
    }

    let output = draws
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(path) = args.path {
        std::fs::write(&path, output).context(format!("Failed to write results to {:?}", path))?;
        println!("\nResults saved to: {:?}", path);
    } else {
        println!("\nGenerated Draws:\n{}", output);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // This allows the tests to "see" the code in the main file
    use super::*;
    use rand::SeedableRng; // Make sure this trait is in scope for .from_seed()

    #[test]
    fn test_draw_generation() {
        let mut rng = Pcg64::from_seed([0; 32]);
        let draw = generate_draw(&mut rng).expect("Generation failed");

        assert_eq!(draw.white_balls.len(), 5);
        assert!(draw.white_balls.windows(2).all(|w| w[0] < w[1]));
        assert!(draw.white_balls.iter().all(|&n| (1..=70).contains(&n)));
        assert!(draw.mega_ball >= 1 && draw.mega_ball <= 25);
    }
}
