use anyhow::{Result, bail};
use rand::Rng;
use rand::seq::SliceRandom;

pub struct Grid {
    pub size: usize,
    pub cells: Vec<Vec<String>>,
}

/// Number of cells that must come from the element pool: the whole grid,
/// minus one for the free space when requested.
pub fn required_elements(size: usize, free_space: bool) -> Result<usize> {
    if size == 0 {
        bail!("grid size must be greater than zero");
    }
    if free_space && size.is_multiple_of(2) {
        bail!("--free-space requires an odd grid size, got {size}");
    }

    let total = size * size;
    Ok(if free_space { total - 1 } else { total })
}

/// Build a single bingo grid by drawing a random, non-repeating sample of
/// `required_elements(size, free_space)` items from `elements`.
pub fn generate<R: Rng + ?Sized>(
    elements: &[String],
    size: usize,
    free_space: Option<&str>,
    rng: &mut R,
) -> Result<Grid> {
    let needed = required_elements(size, free_space.is_some())?;
    if elements.len() < needed {
        bail!(
            "not enough elements: need at least {needed} to fill a {size}x{size} grid{}, got {}",
            if free_space.is_some() {
                " with a free space"
            } else {
                ""
            },
            elements.len()
        );
    }

    let mut pool: Vec<&String> = elements.iter().collect();
    pool.shuffle(rng);
    let mut drawn = pool.into_iter().take(needed);

    let center = size / 2;
    let mut cells = Vec::with_capacity(size);
    for row in 0..size {
        let mut line = Vec::with_capacity(size);
        for col in 0..size {
            if let Some(text) = free_space
                && row == center
                && col == center
            {
                line.push(text.to_string());
                continue;
            }
            line.push(
                drawn
                    .next()
                    .expect("pool was sized to fit the grid")
                    .clone(),
            );
        }
        cells.push(line);
    }

    Ok(Grid { size, cells })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn elements(n: usize) -> Vec<String> {
        (1..=n).map(|i| i.to_string()).collect()
    }

    #[test]
    fn required_elements_without_free_space() {
        assert_eq!(required_elements(5, false).unwrap(), 25);
        assert_eq!(required_elements(3, false).unwrap(), 9);
    }

    #[test]
    fn required_elements_with_free_space() {
        assert_eq!(required_elements(5, true).unwrap(), 24);
    }

    #[test]
    fn free_space_rejects_even_size() {
        assert!(required_elements(4, true).is_err());
    }

    #[test]
    fn zero_size_is_rejected() {
        assert!(required_elements(0, false).is_err());
    }

    #[test]
    fn generate_fills_grid_without_free_space() {
        let mut rng = StdRng::seed_from_u64(42);
        let els = elements(25);
        let grid = generate(&els, 5, None, &mut rng).unwrap();
        assert_eq!(grid.size, 5);
        assert_eq!(grid.cells.len(), 5);
        assert!(grid.cells.iter().all(|row| row.len() == 5));
    }

    #[test]
    fn generate_places_free_space_in_center() {
        let mut rng = StdRng::seed_from_u64(1);
        let els = elements(24);
        let grid = generate(&els, 5, Some("FREE"), &mut rng).unwrap();
        assert_eq!(grid.cells[2][2], "FREE");
    }

    #[test]
    fn generate_errors_when_not_enough_elements() {
        let mut rng = StdRng::seed_from_u64(1);
        let els = elements(10);
        assert!(generate(&els, 5, None, &mut rng).is_err());
    }

    #[test]
    fn generate_is_deterministic_with_seed() {
        let els = elements(25);
        let mut rng_a = StdRng::seed_from_u64(7);
        let mut rng_b = StdRng::seed_from_u64(7);
        let a = generate(&els, 5, None, &mut rng_a).unwrap();
        let b = generate(&els, 5, None, &mut rng_b).unwrap();
        assert_eq!(a.cells, b.cells);
    }
}
