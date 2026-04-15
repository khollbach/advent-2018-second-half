use anyhow::bail;

pub struct Grid {
    cells: Vec<Vec<Cell>>,
}

enum Cell {
    Empty,
    Filled,
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
    pub fn pour_water(&mut self, source: Point) -> Result<PourWaterRet> {
        // Fall downwards.
        let mut curr = source;
        loop {
            let mut next = curr;
            next.y += 1;

            if next.y >= self.dims().y {
                return Ok(PourWaterRet::Done); // Fell off the bottom.
            }

            if self.get(next) == Cell::Filled {
                break; // Hit a block.
            }

            curr = next;
        }

        let (left, leftRet) = self.go_left(curr)?;
        let (right, rightRet) = self.go_right(curr)?;
        if leftRet == GoLeftOrRightRet::Blocked && rightRet == GoLeftOrRightRet::Blocked {
            for x in left.x..=right.x {
                let p = Point { x, y: left: y};
                self.cells.set(p, Cell::Filled);
                return Ok(PourWaterRet::StartOver);
            }
        }

        if leftRet == GoLeftOrRightRet::Fall {
            if self.pour_water(left)? == PourWaterRet::StartOver {
                return Ok(PourWaterRet::StartOver);
            }
        }
        if rightRet == GoLeftOrRightRet::Fall {
            if self.pour_water(right)? == PourWaterRet::StartOver {
                return Ok(PourWaterRet::StartOver);
            }
        }

        Ok(PourWaterRet::Done)
    }

    fn go_left(&mut self, p: Point) -> Result<(Point, GoLeftOrRightRet)> {
        loop {
            // Can fall down?
            let mut below = curr;
            below.y += 1;
            if below.y >= self.dims().y || self.get(below) == Cell::Empty {
                return Ok((curr, GoLeftOrRightRet::Fall));
            }

            let mut next = curr;
            next.x -= 1;
            if next.x < 0 {
                bail!("fell off the side");
            }

            // Blocked?
            if self.get(next) == Cell::Filled {
                return Ok((curr, GoLeftOrRightRet::Blocked));
            }

            curr = next;
        }
    }
}

enum PourWaterRet {
    StartOver,
    Done,
}

enum GoLeftOrRightRet {
    Blocked,
    Fall,
}

struct Point {
    x: i32,
    y: i32,
}
