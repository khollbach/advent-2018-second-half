use std::{
    collections::HashMap,
    fmt,
    hash::{Hash, Hasher},
    io, mem,
};

use anyhow::Result;

mod input;

fn main() -> Result<()> {
    let input = input::parse(io::stdin().lock())?;

    let mut world = input.clone();
    for _ in 0..10 {
        world.step();
    }
    println!("{}", world.count_trees() * world.count_lumber());

    let mut world = input;
    let (num_steps, cycle_len) = find_repeat(&mut world);
    let remaining = 1_000_000_000 - num_steps;
    for _ in 0..remaining % cycle_len {
        world.step();
    }
    println!("{}", world.count_trees() * world.count_lumber());

    Ok(())
}

/// Return `(num_steps, cycle_len)`.
fn find_repeat(world: &mut World) -> (usize, usize) {
    let mut seen = HashMap::new();
    for i in 0.. {
        if let Some(&prev_idx) = seen.get(world) {
            let len = i - prev_idx;
            return (prev_idx, len);
        }
        seen.insert(world.clone(), i);

        world.step();
    }
    unreachable!()
}

#[derive(Clone)]
struct World {
    cells: Vec<Vec<Cell>>,
    tmp_buf: Vec<Vec<Cell>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Trees,
    Lumber,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: isize,
    col: isize,
}

fn dims_contains(dims: Point, p: Point) -> bool {
    let row = (0..dims.row).contains(&p.row);
    let col = (0..dims.col).contains(&p.col);
    row && col
}

impl World {
    /// `(num_rows, num_cols)`
    fn dims(&self) -> Point {
        let row = self.cells.len();
        if row == 0 {
            return Point { row: 0, col: 0 };
        }
        let col = self.cells[0].len();
        Point {
            row: row.try_into().unwrap(),
            col: col.try_into().unwrap(),
        }
    }

    fn in_bounds(&self, p: Point) -> bool {
        dims_contains(self.dims(), p)
    }

    fn get(&self, p: Point) -> Cell {
        assert!(self.in_bounds(p));
        self.cells[p.row as usize][p.col as usize]
    }

    fn set_tmp_buf(&mut self, p: Point, cell: Cell) {
        assert!(self.in_bounds(p));
        self.tmp_buf[p.row as usize][p.col as usize] = cell;
    }

    fn points(&self) -> impl Iterator<Item = Point> + 'static {
        let dims = self.dims();
        (0..dims.row).flat_map(move |row| (0..dims.col).map(move |col| Point { row, col }))
    }

    fn neighbors(&self, p: Point) -> impl Iterator<Item = Point> {
        assert!(self.in_bounds(p));

        [-1, 0, 1].into_iter().flat_map(move |d_row| {
            [-1, 0, 1].into_iter().filter_map(move |d_col| {
                if (d_row, d_col) == (0, 0) {
                    return None;
                }
                let p2 = Point {
                    row: p.row + d_row,
                    col: p.col + d_col,
                };
                if !self.in_bounds(p2) {
                    return None;
                }
                Some(p2)
            })
        })
    }
}

impl World {
    fn step(&mut self) {
        self.generate_next();
        mem::swap(&mut self.cells, &mut self.tmp_buf);
    }

    fn generate_next(&mut self) {
        for p in self.points() {
            let cell = self.next_cell(p);
            self.set_tmp_buf(p, cell);
        }
    }

    fn next_cell(&self, p: Point) -> Cell {
        let mut cell = self.get(p);
        match cell {
            Cell::Empty => {
                if self.adj_trees(p) >= 3 {
                    cell = Cell::Trees;
                }
            }
            Cell::Trees => {
                if self.adj_lumber(p) >= 3 {
                    cell = Cell::Lumber;
                }
            }
            Cell::Lumber => {
                if self.adj_lumber(p) >= 1 && self.adj_trees(p) >= 1 {
                    // Stay the same.
                } else {
                    cell = Cell::Empty;
                }
            }
        };
        cell
    }

    fn adj_trees(&self, p: Point) -> usize {
        self.neighbors(p)
            .filter(|&p2| self.get(p2) == Cell::Trees)
            .count()
    }

    fn adj_lumber(&self, p: Point) -> usize {
        self.neighbors(p)
            .filter(|&p2| self.get(p2) == Cell::Lumber)
            .count()
    }
}

impl World {
    fn count_trees(&self) -> usize {
        self.points()
            .filter(|&p| self.get(p) == Cell::Trees)
            .count()
    }

    fn count_lumber(&self) -> usize {
        self.points()
            .filter(|&p| self.get(p) == Cell::Lumber)
            .count()
    }
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for p in self.points() {
            if p.row != 0 && p.col == 0 {
                writeln!(f)?;
            }
            let c = match self.get(p) {
                Cell::Empty => '.',
                Cell::Trees => '|',
                Cell::Lumber => '#',
            };
            write!(f, "{}", c)?;
        }
        writeln!(f)
    }
}

impl Hash for World {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cells.hash(state);
    }
}

impl PartialEq for World {
    fn eq(&self, other: &Self) -> bool {
        self.cells == other.cells
    }
}

impl Eq for World {}
