use std::io::BufRead;

use anyhow::{Context, Result, bail};

use crate::grid::{Cell, Grid, Point};

#[derive(Debug, Clone, Copy)]
enum Line {
    Vertical { y_min: u32, y_max: u32, x: u32 },
    Horizontal { x_min: u32, x_max: u32, y: u32 },
}

pub fn parse<R: BufRead>(r: R) -> Result<Grid> {
    let mut lines = vec![];
    for l in r.lines() {
        let l = l?;
        lines.push(parse_line(&l)?);
    }

    // todo: this feels like it could be made simpler somehow

    let mut max_x = 0;
    let mut max_y = 0;
    for &line in &lines {
        match line {
            Line::Vertical { y_min: _, y_max, x } => {
                max_x = max_x.max(x);
                max_y = max_y.max(y_max);
            }
            Line::Horizontal { x_min: _, x_max, y } => {
                max_x = max_x.max(x_max);
                max_y = max_y.max(y);
            }
        }
    }
    let dims = Point {
        x: 1 + i32::try_from(max_x).context("too large")?,
        y: 1 + i32::try_from(max_y).context("too large")?,
    };
    let mut grid = Grid::new(dims);

    for line in lines {
        match line {
            Line::Vertical { y_min, y_max, x } => {
                for y in y_min..=y_max {
                    let x = i32::try_from(x).context("u32 too large")?;
                    let y = i32::try_from(y).context("u32 too large")?;
                    let p = Point { x, y };
                    grid.set(p, Cell::Clay);
                }
            }
            Line::Horizontal { x_min, x_max, y } => {
                for x in x_min..=x_max {
                    let x = i32::try_from(x).context("u32 too large")?;
                    let y = i32::try_from(y).context("u32 too large")?;
                    let p = Point { x, y };
                    grid.set(p, Cell::Clay);
                }
            }
        }
    }

    Ok(grid)
}

fn parse_line(s: &str) -> Result<Line> {
    let (left, right) = s.split_once(", ").context("expected `, `")?;
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

    let ret = if vertical {
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
    };
    Ok(ret)
}
