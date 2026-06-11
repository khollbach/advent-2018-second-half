use std::{
    io,
    ops::{Add, Mul, Sub},
};

use anyhow::Result;

mod input;
mod part_2;

fn main() -> Result<()> {
    let nanobots = input::parse(io::stdin().lock())?;
    let ans = part_1(&nanobots);
    println!("{}", ans);
    let ans = part_2::solve(&nanobots);
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

    fn corners(self) -> [Point; 6] {
        let p = self.pos;
        let r = self.range;
        [
            p + Point::X * r,
            p + Point::X * -r,
            p + Point::Y * r,
            p + Point::Y * -r,
            p + Point::Z * r,
            p + Point::Z * -r,
        ]
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

    const X: Self = Self { x: 1, y: 0, z: 0 };
    const Y: Self = Self { x: 0, y: 1, z: 0 };
    const Z: Self = Self { x: 0, y: 0, z: 1 };
    const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];
    const ORIGIN: Self = Self { x: 0, y: 0, z: 0 };
    const ONES: Self = Self { x: 1, y: 1, z: 1 };
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
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

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, scale: i32) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}
