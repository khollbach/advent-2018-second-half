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

---

Wait a SECOND. I'm trolling. These are not even cubes.
But maybe we could apply a transformation (rotation?) to the input space
so that they become cubes?
... nope. they've got only 6 pointy bits, not 8...
- they're kinda like two 4-sided pyramids glued together. I'm sure there's a term for this.

---

Let's just try corners-only and see what the online judge says :)
    [day-23/src/part_2.rs:62:5] best = Point {
        x: 62699654,
        y: 21730841,
        z: 24154493,
    }
    [day-23/src/part_2.rs:63:5] hit_count(nanobots, best) = 894
    ans: 108584988 (manhattan norm)
verdict: "not right; too high"
(note: centers-only best is 856 -- lower)

---

But 894 being a lower-bound on the max-hitcount is quite high!
Is there a way to make use of this?
[ ] TODO: keep thinking

---

Hmmm. This is posed as discrete, but at the same time, some sort of gradient
descent feels like it could do well at optimizing...

---

I should really just render this, either in 3D, or as 3 2D projects, and LOOK at it.
There's a good chance some sort of not-by-chance structure will jump out of this...

*/

use std::cmp::Reverse;

use itertools::Itertools;

use crate::{Nanobot, Point};

pub fn solve(nanobots: &[Nanobot]) -> i32 {
    // _greedy_search(nanobots)
    brute_force_nearby(nanobots)
    // testing(nanobots)
}

fn _heuristic(nanobots: &[Nanobot]) -> i32 {
    let corners: Vec<_> = nanobots.iter().flat_map(|n| n.corners()).collect();

    let best = corners
        .into_iter()
        .max_by_key(|&p| (hit_count(nanobots, p), Reverse(p.manhattan_norm())))
        .unwrap();

    // let centers: Vec<_> = nanobots.iter().map(|n| n.pos).collect();
    // let mut avg = Point::ORIGIN;
    // for p in centers {
    //     avg = avg + p;
    // }
    // avg.x /= i32::try_from(nanobots.len()).unwrap();
    // avg.y /= i32::try_from(nanobots.len()).unwrap();
    // avg.z /= i32::try_from(nanobots.len()).unwrap();
    // dbg!(hit_count(nanobots, avg)); // 85

    // dbg!(best);
    // dbg!(hit_count(nanobots, best));
    best.manhattan_norm()
}

fn hit_count(nanobots: &[Nanobot], p: Point) -> usize {
    nanobots.iter().filter(|n| n.in_range(p)).count()
}

/// "gradient descent" (??)
fn _greedy_search(nanobots: &[Nanobot]) -> i32 {
    // let mut curr = Point::ORIGIN; // TODO: ?

    // The best corner.
    let mut curr = Point {
        x: 62699654,
        y: 21730841,
        z: 24154493,
    };

    dbg!(hit_count(nanobots, curr), curr);

    // hmmm..
    // Maybe we need a concept of momentum, to prevent it from getting stuck?

    for i in 0.. {
        let next = curr
            .search_neighbors()
            .into_iter()
            .max_by_key(|&p| hit_count(nanobots, p))
            .unwrap();

        if hit_count(nanobots, next) <= hit_count(nanobots, curr) {
            println!("local minimum at {:?} after {} iterations", curr, i + 1);
            return curr.manhattan_norm();
        }

        dbg!(hit_count(nanobots, next), next);
        curr = next;
    }

    unreachable!()
}

fn testing(nanobots: &[Nanobot]) -> i32 {
    let (xs, ys, zs) = nanobots
        .iter()
        .map(
            |Nanobot {
                 pos: Point { x, y, z },
                 ..
             }| (x, y, z),
        )
        .multiunzip();
    print_bounds(&xs);
    print_bounds(&ys);
    print_bounds(&zs);

    let all_corners: Vec<_> = nanobots.iter().flat_map(|n| n.corners()).collect();
    let best = all_corners
        .iter()
        .copied()
        .max_by_key(|&p| (hit_count(nanobots, p), Reverse(p.manhattan_norm())))
        .unwrap();
    let best_corners: Vec<_> = nanobots
        .iter()
        .flat_map(|n| n.corners())
        .filter(|&c| hit_count(nanobots, c) == hit_count(nanobots, best))
        .collect();
    dbg!(hit_count(nanobots, best), &best_corners, best_corners.len());

    let mut scores = all_corners
        .into_iter()
        .map(|p| (hit_count(nanobots, p), Reverse(p.manhattan_norm()), p))
        .collect_vec();
    scores.sort_by_key(|&x| Reverse(x.0));
    dbg!(&scores[..10]);
    let (xs, ys, zs) = scores[..2]
        .iter()
        .map(|(_, _, Point { x, y, z })| (x, y, z))
        .multiunzip();
    print_bounds(&xs);
    print_bounds(&ys);
    print_bounds(&zs);

    0
}

fn print_bounds(xs: &Vec<i32>) {
    let min = xs.iter().min().unwrap();
    let max = xs.iter().max().unwrap();
    println!("{}..={} ({})", min, max, max - min);
}

fn brute_force_nearby(nanobots: &[Nanobot]) -> i32 {
    let mut best_corner = Point {
        x: 62699654,
        y: 21730841,
        z: 24154493,
    };

    let mut best = best_corner;
    let mut count = 0;
    for x in -50..=50 {
        for y in -50..=50 {
            for z in -50..=50 {
                let d = Point { x, y, z };
                let p = best_corner + d;
                // if hit_count(nanobots, p) > hit_count(nanobots, best) {
                if hit_count(nanobots, p) == 901 {
                    // dbg!(p, hit_count(nanobots, p));
                    count += 1;
                    // best = p;
                }
            }
        }
    }
    dbg!(count);

    todo!()
}

/*
interestingly by just guessing something near our previous best, we got something better!
    [day-23/src/part_2.rs:119:5] hit_count(nanobots, curr) = 901
    [day-23/src/part_2.rs:119:5] curr = Point {
        x: 62699650,
        y: 21730840,
        z: 24154490,
    }
*/

impl Point {
    fn search_neighbors(self) -> Vec<Self> {
        let mut out = vec![];
        for dirn in Point::AXES {
            for magnitude_log in 0..24 {
                let magnitude = 1 << magnitude_log;
                out.push(dirn * magnitude);
                out.push(dirn * -magnitude);
            }
        }
        out
    }
}
