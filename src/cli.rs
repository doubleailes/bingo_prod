use std::path::PathBuf;

use clap::{Parser, ValueEnum};

/// Generate bingo grids from a list of elements.
#[derive(Parser, Debug)]
#[command(name = "bingo", version, about, long_about = None)]
pub struct Args {
    /// Elements to place on the grid, given directly on the command line.
    ///
    /// Ignored if `--input` is provided.
    pub elements: Vec<String>,

    /// Read elements from a file instead (one element per line).
    #[arg(short, long, value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Size of the grid (produces an N x N grid).
    #[arg(short, long, default_value_t = 5)]
    pub size: usize,

    /// Number of bingo cards to generate.
    #[arg(short, long, default_value_t = 1)]
    pub count: usize,

    /// Insert a free space in the center of the grid (requires an odd size).
    #[arg(short = 'f', long)]
    pub free_space: bool,

    /// Text to display in the free space.
    #[arg(long, default_value = "FREE", value_name = "TEXT")]
    pub free_space_text: String,

    /// Seed the random number generator for reproducible grids.
    #[arg(long, value_name = "SEED")]
    pub seed: Option<u64>,

    /// Output format.
    #[arg(short = 'F', long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,

    /// Write the output to a file instead of stdout.
    ///
    /// Required when `--format png`, since image data can't be written to
    /// stdout. When generating more than one card, each card is written
    /// next to this path with a `-<N>` suffix inserted before the extension.
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Text,
    Csv,
    Markdown,
    /// Render each card as a PNG image instead of text.
    Png,
}
