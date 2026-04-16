use std::collections::HashSet;

use anyhow::{Result, bail};

pub struct Grid {
    /// Non-empty.
    cells: Vec<Vec<Cell>>,

    wet: HashSet<Point>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Sand,
    Water,
    Clay,
}

/*
ok let's start with pseudocode

- go down until you can't (if you fall off; die)
    (o/w; you've hit a block)

- go left until
    - you hit a block
    - or there's a space below you
- same for right
- if both are blocks
    then fill the space in between and start over
    (from the very beginning!)

- o/w, for any spaces below; recursively pour water from there
    - if either says "start over", do so

---

TODO: next steps
- impl "PrintGrid"
- fix compiler errors
- use Print to test the before/after on the small input

- if this *does* actually work, it might be nice to clean up the code

*/

impl Grid {
    // todo: better way to split up cxtor fxnality b/w here and input?
    /// Dimensions must be positive.
    pub fn new(dims: Point) -> Self {
        assert!(dims.x > 0);
        assert!(dims.y > 0);
        let y = usize::try_from(dims.y).unwrap();
        let x = usize::try_from(dims.x).unwrap();
        Self {
            cells: vec![vec![Cell::Sand; x]; y],
            wet: HashSet::new(),
        }
    }

    fn get(&self, p: Point) -> Cell {
        let y = usize::try_from(p.y).unwrap();
        let x = usize::try_from(p.x).unwrap();
        self.cells[y][x]
    }

    pub fn set(&mut self, p: Point, cell: Cell) {
        let y = usize::try_from(p.y).unwrap();
        let x = usize::try_from(p.x).unwrap();
        self.cells[y][x] = cell;
    }

    // todo: not pub
    pub fn count_wet_part_1(&self) -> usize {
        let bb = self.bounding_box();
        self.wet
            .iter()
            .filter(|p| bb.min.y <= p.y && p.y < bb.max.y)
            .count()
    }

    // todo: not pub
    pub fn count_water_part_2(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|row| row.iter().filter(|&&cell| cell == Cell::Water))
            .count()
    }

    // todo: fn order
    /// What is the smallest box that contains all clay?
    ///
    /// Note: min values are inclusive, and max values are exclusive.
    fn bounding_box(&self) -> BoundingBox {
        let filled = self.points().filter(|&p| self.get(p) == Cell::Clay);
        BoundingBox {
            min: Point {
                x: filled.clone().map(|p| p.x).min().unwrap(),
                y: filled.clone().map(|p| p.y).min().unwrap(),
            },
            max: Point {
                x: 1 + filled.clone().map(|p| p.x).max().unwrap(),
                y: 1 + filled.clone().map(|p| p.y).max().unwrap(),
            },
        }
    }

    /// For debugging.
    pub fn print(&self) {
        let bb = self.bounding_box();

        for y in bb.min.y..bb.max.y {
            for x in bb.min.x..bb.max.x {
                let p = Point { x, y };
                let c = if self.get(p) == Cell::Clay {
                    '#'
                } else if self.get(p) == Cell::Water {
                    '~'
                } else if self.wet.contains(&p) {
                    '|'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
    }

    // todo: method ordering ?
    fn dims(&self) -> Point {
        let y = self.cells.len();
        let x = self.cells[0].len();

        let x = i32::try_from(x).unwrap();
        let y = i32::try_from(y).unwrap();

        Point { x, y }
    }

    fn points(&self) -> impl Iterator<Item = Point> + Clone {
        let dims = self.dims();
        (0..dims.y).flat_map(move |y| (0..dims.x).map(move |x| Point { x, y }))
    }

    pub fn pour_water(&mut self, source: Point) -> Result<()> {
        while let PourWaterRet::StartOver = self.pour_water_helper(source)? {}
        Ok(())
    }

    fn pour_water_helper(&mut self, source: Point) -> Result<PourWaterRet> {
        self.wet.insert(source);

        // Fall downwards.
        let mut curr = source;
        loop {
            let mut next = curr;
            next.y += 1;

            if next.y >= self.dims().y {
                return Ok(PourWaterRet::Done); // Fell off the bottom.
            }

            if self.get(next) != Cell::Sand {
                break; // Hit a block.
            }

            curr = next;
            self.wet.insert(curr);
        }

        let (left, left_ret) = self.go_left(curr)?;
        let (right, right_ret) = self.go_right(curr)?;
        if left_ret == GoLeftOrRightRet::Blocked && right_ret == GoLeftOrRightRet::Blocked {
            for x in left.x..=right.x {
                let p = Point { x, y: left.y };
                self.set(p, Cell::Water);
            }
            return Ok(PourWaterRet::StartOver);
        }

        if left_ret == GoLeftOrRightRet::Fall {
            if self.pour_water_helper(left)? == PourWaterRet::StartOver {
                return Ok(PourWaterRet::StartOver);
            }
        }
        if right_ret == GoLeftOrRightRet::Fall {
            if self.pour_water_helper(right)? == PourWaterRet::StartOver {
                return Ok(PourWaterRet::StartOver);
            }
        }

        Ok(PourWaterRet::Done)
    }

    fn go_left(&mut self, p: Point) -> Result<(Point, GoLeftOrRightRet)> {
        let mut curr = p;
        loop {
            // Can fall down?
            let mut below = curr;
            below.y += 1;
            if below.y >= self.dims().y || self.get(below) == Cell::Sand {
                return Ok((curr, GoLeftOrRightRet::Fall));
            }

            let mut next = curr;
            next.x -= 1;
            if next.x < 0 {
                bail!("fell off the side");
            }

            // Blocked?
            if self.get(next) == Cell::Clay {
                return Ok((curr, GoLeftOrRightRet::Blocked));
            }

            curr = next;
            self.wet.insert(curr);
        }
    }

    // todo: dedup code w/ go_left
    fn go_right(&mut self, p: Point) -> Result<(Point, GoLeftOrRightRet)> {
        let mut curr = p;
        loop {
            // Can fall down?
            let mut below = curr;
            below.y += 1;
            if below.y >= self.dims().y || self.get(below) == Cell::Sand {
                return Ok((curr, GoLeftOrRightRet::Fall));
            }

            let mut next = curr;
            next.x += 1;
            if next.x >= self.dims().x {
                bail!("fell off the side");
            }

            // Blocked?
            if self.get(next) == Cell::Clay {
                return Ok((curr, GoLeftOrRightRet::Blocked));
            }

            curr = next;
            self.wet.insert(curr);
        }
    }
}

// todo: make not pub, and wrap pour_water w/ helper method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PourWaterRet {
    StartOver,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GoLeftOrRightRet {
    Blocked,
    Fall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

struct BoundingBox {
    /// Inclusive.
    min: Point,
    /// Exclusive.
    max: Point,
}
