use std::io::Cursor;

use anyhow::Result;
use image::{ImageFormat, Rgb, RgbImage};
use noto_sans_mono_bitmap::{FontWeight, RasterHeight, get_raster, get_raster_width};

use crate::grid::Grid;

const FONT_WEIGHT: FontWeight = FontWeight::Regular;
const FONT_SIZE: RasterHeight = RasterHeight::Size20;
const MAX_LINE_CHARS: usize = 12;
const CELL_PADDING: u32 = 12;
const LINE_SPACING: u32 = 2;
const GRID_LINE_THICKNESS: u32 = 2;

const BACKGROUND: Rgb<u8> = Rgb([255, 255, 255]);
const GRID_LINE: Rgb<u8> = Rgb([0, 0, 0]);

/// Render a grid as a PNG image, returning the encoded bytes.
pub fn render(grid: &Grid) -> Result<Vec<u8>> {
    let char_width = get_raster_width(FONT_WEIGHT, FONT_SIZE) as u32;
    let char_height = FONT_SIZE.val() as u32;

    let wrapped: Vec<Vec<String>> = grid
        .cells
        .iter()
        .flatten()
        .map(|text| wrap(text, MAX_LINE_CHARS))
        .collect();

    let max_line_chars = wrapped
        .iter()
        .flatten()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(1)
        .max(1) as u32;
    let max_lines = wrapped.iter().map(Vec::len).max().unwrap_or(1).max(1) as u32;

    let inner_w = max_line_chars * char_width;
    let inner_h = max_lines * char_height + (max_lines - 1) * LINE_SPACING;
    let cell_w = inner_w + 2 * CELL_PADDING;
    let cell_h = inner_h + 2 * CELL_PADDING;

    let size = grid.size as u32;
    let image_w = GRID_LINE_THICKNESS + size * (cell_w + GRID_LINE_THICKNESS);
    let image_h = GRID_LINE_THICKNESS + size * (cell_h + GRID_LINE_THICKNESS);

    let mut img = RgbImage::from_pixel(image_w, image_h, BACKGROUND);
    draw_grid_lines(&mut img, size, cell_w, cell_h);

    for (index, lines) in wrapped.iter().enumerate() {
        let row = index as u32 / size;
        let col = index as u32 % size;
        let cell_x = GRID_LINE_THICKNESS + col * (cell_w + GRID_LINE_THICKNESS);
        let cell_y = GRID_LINE_THICKNESS + row * (cell_h + GRID_LINE_THICKNESS);
        draw_cell_text(
            &mut img,
            lines,
            cell_x,
            cell_y,
            inner_w,
            inner_h,
            char_width,
            char_height,
        );
    }

    let mut buf = Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(img).write_to(&mut buf, ImageFormat::Png)?;
    Ok(buf.into_inner())
}

fn draw_grid_lines(img: &mut RgbImage, size: u32, cell_w: u32, cell_h: u32) {
    let width = img.width();
    let height = img.height();

    for i in 0..=size {
        let x = i * (cell_w + GRID_LINE_THICKNESS);
        fill_rect(img, x, 0, GRID_LINE_THICKNESS, height, GRID_LINE);
        let y = i * (cell_h + GRID_LINE_THICKNESS);
        fill_rect(img, 0, y, width, GRID_LINE_THICKNESS, GRID_LINE);
    }
}

fn fill_rect(img: &mut RgbImage, x0: u32, y0: u32, w: u32, h: u32, color: Rgb<u8>) {
    for y in y0..(y0 + h).min(img.height()) {
        for x in x0..(x0 + w).min(img.width()) {
            img.put_pixel(x, y, color);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_cell_text(
    img: &mut RgbImage,
    lines: &[String],
    cell_x: u32,
    cell_y: u32,
    inner_w: u32,
    inner_h: u32,
    char_width: u32,
    char_height: u32,
) {
    let block_h =
        lines.len() as u32 * char_height + (lines.len() as u32).saturating_sub(1) * LINE_SPACING;
    let mut y = cell_y + CELL_PADDING + (inner_h.saturating_sub(block_h)) / 2;

    for line in lines {
        let line_w = line.chars().count() as u32 * char_width;
        let mut x = cell_x + CELL_PADDING + (inner_w.saturating_sub(line_w)) / 2;
        for c in line.chars() {
            draw_char(img, x, y, c);
            x += char_width;
        }
        y += char_height + LINE_SPACING;
    }
}

fn draw_char(img: &mut RgbImage, x: u32, y: u32, c: char) {
    let Some(raster) = get_raster(c, FONT_WEIGHT, FONT_SIZE) else {
        return;
    };

    for (row_i, row) in raster.raster().iter().enumerate() {
        for (col_i, &intensity) in row.iter().enumerate() {
            if intensity == 0 {
                continue;
            }
            let px = x + col_i as u32;
            let py = y + row_i as u32;
            if px < img.width() && py < img.height() {
                let shade = 255 - intensity;
                img.put_pixel(px, py, Rgb([shade, shade, shade]));
            }
        }
    }
}

/// Word-wrap `text` into lines of at most `max_chars` characters, hard-breaking
/// any single word that is longer than `max_chars` on its own.
fn wrap(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for chunk in split_into_chunks(text, max_chars) {
        if current.is_empty() {
            current = chunk;
        } else if current.chars().count() + 1 + chunk.chars().count() <= max_chars {
            current.push(' ');
            current.push_str(&chunk);
        } else {
            lines.push(std::mem::replace(&mut current, chunk));
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

/// Split `text` on whitespace, breaking any word longer than `max_chars` into
/// pieces so every returned chunk fits on one line by itself.
fn split_into_chunks(text: &str, max_chars: usize) -> Vec<String> {
    let max_chars = max_chars.max(1);
    let mut chunks = Vec::new();
    for word in text.split_whitespace() {
        let chars: Vec<char> = word.chars().collect();
        for piece in chars.chunks(max_chars) {
            chunks.push(piece.iter().collect());
        }
    }
    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_grid() -> Grid {
        Grid {
            size: 2,
            cells: vec![
                vec!["a".to_string(), "bb".to_string()],
                vec!["ccc".to_string(), "watermelon".to_string()],
            ],
        }
    }

    #[test]
    fn render_produces_a_valid_png() {
        let bytes = render(&sample_grid()).unwrap();
        assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
    }

    #[test]
    fn wrap_keeps_short_text_on_one_line() {
        assert_eq!(wrap("apple", 12), vec!["apple".to_string()]);
    }

    #[test]
    fn wrap_breaks_long_text_across_lines() {
        assert_eq!(
            wrap("a very long phrase", 8),
            vec![
                "a very".to_string(),
                "long".to_string(),
                "phrase".to_string()
            ]
        );
    }

    #[test]
    fn wrap_hard_breaks_words_longer_than_the_limit() {
        assert_eq!(
            wrap("supercalifragilistic", 6),
            vec![
                "superc".to_string(),
                "alifra".to_string(),
                "gilist".to_string(),
                "ic".to_string(),
            ]
        );
    }

    #[test]
    fn wrap_never_returns_an_empty_vec() {
        assert_eq!(wrap("", 12), vec![String::new()]);
    }
}
