use std::{cmp::Reverse, io, ops::Sub};

use anyhow::Result;
use itertools::Itertools;
use rayon::prelude::*;

mod input;

fn main() -> Result<()> {
    let nanobots = input::parse(io::stdin().lock())?;
    let ans = part_1(&nanobots);
    println!("{}", ans);
    let ans = part_2(&nanobots);
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

/*
part 2 brainstorming
- n=1,000 -- O(n^3) feasible
- size of #s =~ 10,000,000

brute force idea -- O(n^4), but fck it let's try anyways
- project all cubes onto each axis; getting a list of all
    - x border points
    - y border points
    - z border points
- for each x,y,z from those sets, try that candidate point
    (can maybe use rayon for multi-core)
- take the overall best candidate

- 5 seconds for N=100
    - so maybe 50,000 seconds for N=1000 ? -- which could probably finish overnight
- 24 seconds for N=300
- 2 mins, 26 seconds for N=500
    - so maybe only 40 mins for N=1000 ??

---

idea for speeding this up:
- first try all cube corners, and then use the max hit-count of those
    as a lower bound on the actual answer -- call it L
- this should let us avoid checking all border points as follows:
    - take the projection of the cubes onto the x axis
    - find the intervals that have hit-count at least L
        (linear time is possible, but not needed -- can do an n^2 thing here)
        (the hope is that this cuts down the # of border points)
    - (similar for y and z)
    - then feed these reduced x/y/z lists into the existing brute force algo
*/

fn part_2(nanobots: &[Nanobot]) -> i32 {
    let (xs, ys, zs): (Vec<_>, Vec<_>, Vec<_>) = nanobots
        .iter()
        .flat_map(|n| {
            let Point { x, y, z } = n.pos;
            let r = n.range;
            [(x - r, y - r, z - r), (x + r, y + r, z + r)]
        })
        .multiunzip();

    let xs = xs.into_par_iter();
    let ys = ys.into_par_iter();
    let zs = zs.into_par_iter();

    let best = xs
        .flat_map(move |x| {
            let zs = zs.clone();
            ys.clone()
                .flat_map(move |y| zs.clone().map(move |z| Point { x, y, z }))
        })
        .max_by_key(|&p| (hit_count(nanobots, p), Reverse(p.manhattan_norm())))
        .unwrap();

    best.manhattan_norm()
}

fn hit_count(nanobots: &[Nanobot], p: Point) -> usize {
    nanobots.par_iter().filter(|n| n.in_range(p)).count()
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
