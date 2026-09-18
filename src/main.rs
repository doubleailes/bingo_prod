mod cli;
mod grid;
mod output;

use std::fs;
use std::io::Write;

use anyhow::{Context, Result, bail};
use clap::Parser;
use rand::SeedableRng;
use rand::rngs::StdRng;

use cli::Args;

fn main() -> Result<()> {
    let args = Args::parse();
    let elements = load_elements(&args)?;

    let mut rng = match args.seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_rng(&mut rand::rng()),
    };

    let free_space = args.free_space.then_some(args.free_space_text.as_str());

    let mut rendered_cards = Vec::with_capacity(args.count);
    for _ in 0..args.count {
        let grid = grid::generate(&elements, args.size, free_space, &mut rng)?;
        rendered_cards.push(output::render(&grid, args.format));
    }
    let rendered = rendered_cards.join("\n\n");

    match &args.output {
        Some(path) => {
            fs::write(path, rendered + "\n")
                .with_context(|| format!("failed to write output to {}", path.display()))?;
        }
        None => {
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            writeln!(handle, "{rendered}")?;
        }
    }

    Ok(())
}

fn load_elements(args: &Args) -> Result<Vec<String>> {
    let elements = match &args.input {
        Some(path) => {
            let contents = fs::read_to_string(path)
                .with_context(|| format!("failed to read input file {}", path.display()))?;
            contents
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(str::to_string)
                .collect()
        }
        None => args.elements.clone(),
    };

    if elements.is_empty() {
        bail!("no elements provided; pass them as arguments or via --input <FILE>");
    }

    Ok(elements)
}
