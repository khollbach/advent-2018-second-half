use std::io::BufRead;

use anyhow::{Result, bail, ensure};
use itertools::Itertools;

use crate::{Cell, World};

pub fn parse<R: BufRead>(r: R) -> Result<World> {
    let lines: Vec<String> = r.lines().try_collect()?;
    ensure!(!lines.is_empty(), "empty grid");
    ensure!(!lines[0].is_empty(), "empty line");
    for line in &lines {
        ensure!(line.len() == lines[0].len(), "jagged grid");
    }
    let num_rows = lines.len();
    let num_cols = lines[0].len();

    let mut cells = vec![vec![Cell::Empty; num_cols]; num_rows];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            cells[i][j] = char_to_cell(c)?;
        }
    }
    Ok(World {
        tmp_buf: cells.clone(),
        cells,
    })
}

fn char_to_cell(c: char) -> Result<Cell> {
    Ok(match c {
        '.' => Cell::Empty,
        '|' => Cell::Trees,
        '#' => Cell::Lumber,
        _ => bail!("invalid char {c:?}"),
    })
}
