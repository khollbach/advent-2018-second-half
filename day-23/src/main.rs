use std::{io, ops::Sub};

use anyhow::Result;

mod input;

fn main() -> Result<()> {
    let nanobots = input::parse(io::stdin().lock())?;
    let ans = part_1(&nanobots);
    println!("{}", ans);
    Ok(())
}

fn part_1(nanobots: &[Nanobot]) -> i32 {
    assert!(!nanobots.is_empty());
    let strongest = nanobots.iter().max_by_key(|n| n.range).unwrap();

    // (Include `strongest` itself in the count.)
    let mut count = 0;
    for n in nanobots {
        if strongest.in_range(n.pos) {
            count += 1;
        }
    }
    count
}

#[derive(Debug, Clone, Copy)]
struct Nanobot {
    pos: Point,
    /// Manhattan distance.
    range: i32,
}

impl Nanobot {
    fn in_range(self, p: Point) -> bool {
        self.pos.manhattan_distance(p) <= self.range
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn manhattan_distance(self, other: Self) -> i32 {
        (self - other).manhattan_norm()
    }

    fn manhattan_norm(self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
