/*
idea:
- traverse the path (let start=(0,0)); keep track of edges
    - data type: HashSet<(Point, Point)>
- then at the end, to prove we've reconstructed the map,
    do a BFS to find the farthest node from (0, 0)

details:

base cases:
- empty string; easy, path=()

find the top-level split (if exists)
for each of those regexes, recursively traverse them, starting from curr
~~return the union of all of the endpoints~~
for each *mid*point, recursively traverse the rest of the regex
return the union of all *end*points.

if the top-level split doesn't exist,
if the first group is un-paren'd -- just walk the path
    (technically it's possible to do this step-by-step -- since ENWS is just juxtaposing 3 times)
if it's paren'd -- recurse
then for (each) endpoint, recurse on the rest of the input

---

for parsing, we can probably get away with the worst-case n^2 thing,
b/c the input is small (~10,000)

if we want, we can do a proper parse into a tree:
enum Node
    juxtapose (Node, Node)
    bar (Node, Node)
    dirn
    empty

OR, we can probably do something more ad-hoc:
- replace the leading/trailing ^$ with ()
- do a "parsing" pass that produces, for each '('
    - the index of the corresponding ')', AND
    - the indices of the '|'s at this level, if any

^ this sounds fun, so let's try it
*/

use std::{
    cmp::max,
    collections::{HashMap, HashSet, VecDeque},
    io, iter,
};

use anyhow::{Context, Result, ensure};
use itertools::Itertools;

fn main() -> Result<()> {
    let (line,) = io::stdin()
        .lines()
        .collect_tuple()
        .context("expected one line")?;
    let mut regex = line?;

    ensure!(regex.starts_with('^') && regex.ends_with('$'));
    unsafe {
        regex.as_bytes_mut()[0] = b'(';
        *regex.as_bytes_mut().last_mut().unwrap() = b')';
    }

    let paren_groups = parse(&regex);

    let mut edges = HashSet::new();
    traverse(&regex, 0, regex.len(), &paren_groups, (0, 0), &mut edges);

    let (part1, part2) = farthest_point((0, 0), &edges);
    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

type Point = (i32, i32);

fn traverse(
    s: &str,
    i: usize,
    j: usize,
    paren_groups: &HashMap<usize, ParenGroup>,
    start: Point,
    edges: &mut HashSet<(Point, Point)>,
) -> HashSet<Point> {
    assert!(i <= j);
    if i == j {
        return [start].into_iter().collect();
    }

    if s[i..].starts_with('(') {
        let g = &paren_groups[&i];
        let bars = g.top_level_bars.iter().copied();
        let end = g.close_paren;

        let mut midpoints = HashSet::new();
        for (low, high) in iter::once(i).chain(bars).chain([end]).tuple_windows() {
            let points = traverse(s, low + 1, high, paren_groups, start, edges);
            midpoints.extend(points);
        }

        let mut endpoints = HashSet::new();
        for p in midpoints {
            let points = traverse(s, end + 1, j, paren_groups, p, edges);
            endpoints.extend(points);
        }
        endpoints
    } else {
        let dir = match s[i..].chars().next().unwrap() {
            'N' => (0, 1),
            'E' => (1, 0),
            'S' => (0, -1),
            'W' => (-1, 0),
            c => panic!("invalid cardinal direction: {c}"),
        };
        let next = add(start, dir);

        edges.insert((start, next));
        edges.insert((next, start));

        traverse(s, i + 1, j, paren_groups, next, edges)
    }
}

struct ParenGroup {
    close_paren: usize,
    top_level_bars: Vec<usize>,
}

/// Return a map from each open paren to its matching close paren.
fn parse(s: &str) -> HashMap<usize, ParenGroup> {
    assert!(s.starts_with('(') && s.ends_with(')'));

    // A stack of currently open paren groups.
    let mut open_groups = vec![];
    let mut closed_groups = HashMap::new();

    for (i, c) in s.char_indices() {
        match c {
            '(' => open_groups.push((i, vec![])),
            ')' => {
                let (i0, bars) = open_groups.pop().expect("unexpected closing paren");
                let g = ParenGroup {
                    close_paren: i,
                    top_level_bars: bars,
                };
                closed_groups.insert(i0, g);
            }
            '|' => open_groups.last_mut().unwrap().1.push(i),
            _ => (),
        }
    }

    assert!(open_groups.is_empty(), "not enough closing parens");
    closed_groups
}

/// Also return the number of "far" points, where "far" means distance >= 1000.
fn farthest_point(start: Point, edges: &HashSet<(Point, Point)>) -> (u32, usize) {
    let mut seen = HashSet::new();
    let mut to_visit = VecDeque::new();

    seen.insert(start);
    to_visit.push_back((start, 0));

    let mut best = 0;
    let mut count = 0;

    while let Some((p, dist)) = to_visit.pop_front() {
        best = max(best, dist);
        if dist >= 1000 {
            count += 1;
        }

        for p2 in neighbors(p, edges) {
            if !seen.contains(&p2) {
                seen.insert(p2);
                to_visit.push_back((p2, dist + 1));
            }
        }
    }

    (best, count)
}

fn neighbors(p: Point, edges: &HashSet<(Point, Point)>) -> impl Iterator<Item = Point> {
    [(0, -1), (0, 1), (-1, 0), (1, 0)]
        .into_iter()
        .filter_map(move |dir| {
            let p2 = add(p, dir);
            edges.contains(&(p, p2)).then_some(p2)
        })
}

fn add((x, y): Point, (a, b): Point) -> Point {
    (x + a, y + b)
}
