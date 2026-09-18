use crate::cli::OutputFormat;
use crate::grid::Grid;

pub fn render(grid: &Grid, format: OutputFormat) -> String {
    match format {
        OutputFormat::Text => render_text(grid),
        OutputFormat::Csv => render_csv(grid),
        OutputFormat::Markdown => render_markdown(grid),
        OutputFormat::Png => unreachable!("PNG cards are rendered via image_output::render"),
    }
}

fn render_text(grid: &Grid) -> String {
    let width = grid
        .cells
        .iter()
        .flatten()
        .map(|cell| cell.chars().count())
        .max()
        .unwrap_or(0);

    grid.cells
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| format!("{cell:width$}"))
                .collect::<Vec<_>>()
                .join(" | ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_csv(grid: &Grid) -> String {
    grid.cells
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| csv_escape(cell))
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn csv_escape(cell: &str) -> String {
    if cell.contains([',', '"', '\n']) {
        format!("\"{}\"", cell.replace('"', "\"\""))
    } else {
        cell.to_string()
    }
}

fn render_markdown(grid: &Grid) -> String {
    let mut lines = Vec::with_capacity(grid.size + 2);

    let row_to_line = |row: &[String]| format!("| {} |", row.join(" | "));

    if let Some(first) = grid.cells.first() {
        lines.push(row_to_line(first));
        lines.push(format!(
            "|{}|",
            first.iter().map(|_| " --- ").collect::<Vec<_>>().join("|")
        ));
        for row in &grid.cells[1..] {
            lines.push(row_to_line(row));
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_grid() -> Grid {
        Grid {
            size: 2,
            cells: vec![
                vec!["a".to_string(), "bb".to_string()],
                vec!["ccc".to_string(), "d".to_string()],
            ],
        }
    }

    #[test]
    fn csv_render_joins_with_commas() {
        assert_eq!(render_csv(&sample_grid()), "a,bb\nccc,d");
    }

    #[test]
    fn csv_escapes_special_characters() {
        assert_eq!(csv_escape("a,b"), "\"a,b\"");
        assert_eq!(csv_escape("a\"b"), "\"a\"\"b\"");
        assert_eq!(csv_escape("plain"), "plain");
    }

    #[test]
    fn markdown_render_has_header_separator() {
        let out = render_markdown(&sample_grid());
        let mut lines = out.lines();
        assert_eq!(lines.next(), Some("| a | bb |"));
        assert_eq!(lines.next(), Some("| --- | --- |"));
        assert_eq!(lines.next(), Some("| ccc | d |"));
    }
}
