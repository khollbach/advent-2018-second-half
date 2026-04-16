use std::io;

use anyhow::Result;

use crate::{
    grid::{Grid, Tile},
    point::Point,
};

mod grid;
mod input;
mod point;

fn main() -> Result<()> {
    let (grid, min_y, max_y) = input::parse(io::stdin().lock())?;

    let mut sim = Simulation { grid, min_y, max_y };
    sim.drip(Point { x: 500, y: 0 });

    let wet = sim.count_wet();
    let water = sim.count_water();
    println!("{}", wet + water);
    println!("{}", water);

    Ok(())
}

struct Simulation {
    grid: Grid,
    min_y: i64,
    max_y: i64,
}

impl Simulation {
    fn drip(&mut self, p: Point) {
        if p.y > self.max_y {
            return;
        }

        match self.grid.get(p) {
            Tile::Sand => self.grid.set(p, Tile::Wet),
            Tile::Wet => (),
            Tile::Water => return,
            Tile::Rock => panic!(),
        }

        // Can fall down?
        let down = p.down();
        if self.grid.get(down).passable() {
            return self.drip(down);
        }

        // Push water to both sides.
        for side in [p.left(), p.right()] {
            match self.grid.get(side) {
                Tile::Sand => self.drip(side),
                Tile::Wet => (),
                Tile::Water => (),
                Tile::Rock => {
                    if self.grid.get(p) == Tile::Water {
                        // Edge-case: dripping water to the left caused the
                        // current bucket to fill. Don't try to fill it again!
                    } else {
                        if let Some(x_range) = self.detect_bucket(p) {
                            self.fill_bucket(x_range, p.y);
                        }
                    }
                }
            }
        }
    }

    /// If p is at the bottom of a "bucket", return the leftmost and rightmost x
    /// values in the bucket.
    fn detect_bucket(&self, p: Point) -> Option<(i64, i64)> {
        assert!(self.grid.get(p).passable());
        assert!(!self.grid.get(p.down()).passable());

        let mut curr = p;
        while self.grid.get(curr.left()).passable() {
            if self.grid.get(curr.down()).passable() {
                return None; // Would fall.
            }
            curr = curr.left();
        }
        let min_x = curr.x;

        let mut curr = p;
        while self.grid.get(curr.right()).passable() {
            if self.grid.get(curr.down()).passable() {
                return None; // Would fall.
            }
            curr = curr.right();
        }
        let max_x = curr.x;

        Some((min_x, max_x))
    }

    fn fill_bucket(&mut self, (x_min, x_max): (i64, i64), y: i64) {
        // Fill the bottom.
        for x in x_min..=x_max {
            self.grid.set(Point { x, y }, Tile::Water);
        }

        // Re-drip the wet tiles above, now that there's water
        // below them.
        for x in x_min..=x_max {
            let p = Point { x, y };
            let up = p.up();
            if self.grid.get(up) == Tile::Wet {
                self.drip(up);
            }
        }
    }

    fn count_wet(&self) -> usize {
        self.grid.count_wet(self.min_y, self.max_y)
    }

    fn count_water(&self) -> usize {
        self.grid.count_water(self.min_y, self.max_y)
    }
}
