use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

fn main() {
    let depth = 11991;
    let target = (6, 797);
    // let depth = 510;
    // let target = (10, 10);

    let grid = Grid::new(depth, target);
    println!("{}", grid.risk_level(target));

    let ans = grid.shortest_path(
        Point {
            x: 0,
            y: 0,
            plane: Plane::_20,
        },
        Point {
            x: target.0,
            y: target.1,
            plane: Plane::_20,
        },
    );
    println!("{}", ans);
}

/*

- create a grid to store erosion levels
- x,y according to target (+1 to be inclusive)
- populate axes according to rules
    - y=0 => x*16807
    - x=0 => y*48271
- populate each cell according to combinatino rule:
    - mul prev cells, mod 20183
- overwrite target with special "0" value

- print the grid's ".=|" symbols
- return overall sum

*/

struct Grid {
    grid: Vec<Vec<u32>>,
}

impl Grid {
    fn new(depth: u32, target: (usize, usize)) -> Self {
        let mod_ = 20_183;

        let (x, y) = target;
        let num_rows = y + 1;
        let num_cols = y + 1;

        // Storing erosion level (mod mod_).
        let mut grid = vec![vec![0; 1_000]; 1_000]; // todo: something more robust

        grid[0][0] = depth;
        for x in 0..num_cols {
            grid[0][x] = u32::try_from(x).unwrap() * 16_807;
            grid[0][x] += depth;
            grid[0][x] %= mod_;
        }
        for y in 0..num_rows {
            grid[y][0] = u32::try_from(y).unwrap() * 48_271;
            grid[y][0] += depth;
            grid[y][0] %= mod_;
        }

        for y in 1..num_rows {
            for x in 1..num_cols {
                grid[y][x] = grid[y - 1][x] * grid[y][x - 1];
                grid[y][x] += depth;
                grid[y][x] %= mod_;
            }
        }

        // Special case: target.
        grid[y][x] = 0;
        grid[y][x] += depth;
        grid[y][x] %= mod_;

        // Convert to terrain.
        for y in 0..num_rows {
            for x in 0..num_cols {
                grid[y][x] %= 3;
            }
        }

        Self { grid }
    }

    fn _print(&self) {
        for row in &self.grid {
            for x in row {
                let c = match x {
                    0 => '.',
                    1 => '=',
                    2 => '|',
                    _ => unreachable!(),
                };
                print!("{}", c);
            }
            println!();
        }
    }

    fn risk_level(&self, target: (usize, usize)) -> u32 {
        let mut out = 0;

        let (x, y) = target;
        for y in 0..=y {
            for x in 0..=x {
                out += self.grid[y][x];
            }
        }

        out
    }
}

/*

part 2

this feels like it has the flavour of NP-completeness reductions -- where you reshape the input
graph into something you already know what to do with

my idea is to create 3 copies of the input graph:
- one that has rocks and narrows passable (wet is off-limits) -- the "torch" graph
- ...          rocks and wet -- "climbing gear"
- ...          wet and narrow -- "neither"

and then you connect correpsonding coordinates ("vertically") as follows:
- torch-rocks <> climbing-rocks
- torch-narrows <> neither-narrows
- climbing-wet <> neither-wet
with edges of weight 7

To make the code easier to write, let's label rock,wet,narrow as 0,1,2
And let's label climbing,torch,neither as 02,01,12
Note that you start and end in graph 02 -- at coordinates (0,0) and `target` resp.

We're gonna do a BFS (or maybe more like "cost"-first search), and the neighbors function
might not be that complicated, so let's start typing and see what happens

*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Plane {
    _01,
    _12,
    _20,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
    plane: Plane,
}

impl Grid {
    fn shortest_path(&self, start: Point, end: Point) -> u32 {
        let mut seen = HashMap::new();
        let mut to_visit = BinaryHeap::new(); // min-heap

        seen.insert(start, 0);
        to_visit.push((Reverse(0), start));

        while let Some((Reverse(dist), curr)) = to_visit.pop() {
            if dist > seen[&curr] {
                continue;
            }

            if curr == end {
                return dist;
            }

            for (next, cost) in self.neighbors(curr) {
                let next_dist = dist + cost;
                if next_dist < *seen.get(&next).unwrap_or(&u32::MAX) {
                    seen.insert(next, next_dist);
                    to_visit.push((Reverse(next_dist), next));
                }
            }
        }

        panic!("no path found");
    }

    fn neighbors(&self, curr: Point) -> Vec<(Point, u32)> {
        let mut out = vec![];

        let plane = curr.plane.other(self.grid[curr.y][curr.x]);
        out.push((Point { plane, ..curr }, 7));

        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let x = isize::try_from(curr.x).unwrap() + dx;
            let y = isize::try_from(curr.y).unwrap() + dy;
            if self.in_bounds((x, y)) {
                let x = usize::try_from(x).unwrap();
                let y = usize::try_from(y).unwrap();
                if curr.plane.passable(self.grid[y][x]) {
                    out.push((Point { x, y, ..curr }, 1));
                }
            }
        }

        out
    }

    fn in_bounds(&self, (x, y): (isize, isize)) -> bool {
        let y = 0 <= y && y < isize::try_from(self.grid.len()).unwrap();
        let x = 0 <= x && x < isize::try_from(self.grid[0].len()).unwrap();
        x && y
    }
}

impl Plane {
    fn passable(self, tile: u32) -> bool {
        assert!(tile < 3);
        match (self, tile) {
            (Plane::_01, 0 | 1) => true,
            (Plane::_12, 1 | 2) => true,
            (Plane::_20, 2 | 0) => true,
            _ => false,
        }
    }

    fn other(self, tile: u32) -> Self {
        assert!(tile < 3);
        match self {
            Plane::_01 => {
                if tile == 0 {
                    Plane::_20
                } else {
                    Plane::_12
                }
            }
            Plane::_12 => {
                if tile == 1 {
                    Plane::_01
                } else {
                    Plane::_20
                }
            }
            Plane::_20 => {
                if tile == 2 {
                    Plane::_12
                } else {
                    Plane::_01
                }
            }
        }
    }
}
