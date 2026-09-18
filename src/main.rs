mod cli;
mod grid;
mod image_output;
mod output;

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::Parser;
use rand::SeedableRng;
use rand::rngs::StdRng;

use cli::{Args, OutputFormat};
use grid::Grid;

fn main() -> Result<()> {
    let args = Args::parse();
    let elements = load_elements(&args)?;

    let mut rng = match args.seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_rng(&mut rand::rng()),
    };

    let free_space = args.free_space.then_some(args.free_space_text.as_str());

    let mut grids = Vec::with_capacity(args.count);
    for _ in 0..args.count {
        grids.push(grid::generate(&elements, args.size, free_space, &mut rng)?);
    }

    if args.format == OutputFormat::Png {
        write_png_cards(&grids, &args)
    } else {
        write_text_cards(&grids, &args)
    }
}

fn write_text_cards(grids: &[Grid], args: &Args) -> Result<()> {
    let rendered = grids
        .iter()
        .map(|grid| output::render(grid, args.format))
        .collect::<Vec<_>>()
        .join("\n\n");

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

fn write_png_cards(grids: &[Grid], args: &Args) -> Result<()> {
    let base = args
        .output
        .as_ref()
        .context("--output <FILE> is required when using --format png")?;

    for (index, grid) in grids.iter().enumerate() {
        let bytes = image_output::render(grid)?;
        let path = card_path(base, index, grids.len());
        fs::write(&path, bytes)
            .with_context(|| format!("failed to write image to {}", path.display()))?;
    }

    Ok(())
}

/// Path for the `index`-th of `count` cards: `base` unchanged when there's
/// only one card, otherwise `base` with a `-<N>` suffix before the extension.
fn card_path(base: &Path, index: usize, count: usize) -> PathBuf {
    if count <= 1 {
        return base.to_path_buf();
    }

    let stem = base.file_stem().and_then(|s| s.to_str()).unwrap_or("card");
    let mut file_name = format!("{stem}-{}", index + 1);
    if let Some(ext) = base.extension().and_then(|s| s.to_str()) {
        file_name.push('.');
        file_name.push_str(ext);
    }

    let mut path = base.to_path_buf();
    path.set_file_name(file_name);
    path
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
