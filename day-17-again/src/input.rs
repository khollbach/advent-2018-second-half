use std::{
    cmp::{max, min},
    io::BufRead,
};

use anyhow::{Context, Result, bail};

use crate::{
    grid::{Grid, Tile},
    point::Point,
};

/// Returns `(grid, min_y, max_y)`.
pub fn parse<R: BufRead>(r: R) -> Result<(Grid, i64, i64)> {
    let mut grid = Grid::new();
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    let mut insert = |x, y| {
        grid.set(Point { x, y }, Tile::Rock);
        min_y = min(min_y, y);
        max_y = max(max_y, y);
    };

    for l in r.lines() {
        match parse_line(&l?)? {
            Line::Vertical { y_min, y_max, x } => {
                for y in y_min..=y_max {
                    insert(x, y);
                }
            }
            Line::Horizontal { x_min, x_max, y } => {
                for x in x_min..=x_max {
                    insert(x, y);
                }
            }
        }
    }

    Ok((grid, min_y, max_y))
}

#[derive(Debug, Clone, Copy)]
enum Line {
    Vertical { y_min: i64, y_max: i64, x: i64 },
    Horizontal { x_min: i64, x_max: i64, y: i64 },
}

fn parse_line(line: &str) -> Result<Line> {
    let (left, right) = line.split_once(", ").context("expected `, `")?;

    let vertical = if left.starts_with("x=") && right.starts_with("y=") {
        true
    } else if left.starts_with("y=") && right.starts_with("x=") {
        false
    } else {
        bail!("expected either x=..., y=... or y=..., x=...");
    };

    let fixed = left[2..].parse()?;
    let (min, max) = right[2..].split_once("..").context("expected ..")?;
    let min = min.parse()?;
    let max = max.parse()?;

    Ok(if vertical {
        Line::Vertical {
            y_min: min,
            y_max: max,
            x: fixed,
        }
    } else {
        Line::Horizontal {
            x_min: min,
            x_max: max,
            y: fixed,
        }
    })
}
