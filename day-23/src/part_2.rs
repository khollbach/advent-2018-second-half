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

use crate::{Nanobot, Point};

pub fn solve(nanobots: &[Nanobot]) -> i32 {
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
