use crate::Nanobot;

type InputPoint = crate::Point;

/// Make everything positive, so it's easier to work with.
#[derive(Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl From<InputPoint> for Point {
    fn from(p: InputPoint) -> Self {
        Self {
            x: i32_to_u32(p.x),
            y: i32_to_u32(p.y),
            z: i32_to_u32(p.z),
        }
    }
}

/// Map `-2^31..2^31` to `0..2^32`, by adding `2^31`.
fn i32_to_u32(x: i32) -> u32 {
    x.wrapping_sub(i32::MIN) as u32
}

#[derive(Debug, Clone, Copy)]
struct Sphere {
    center: Point,
    radius: u32,
}

/// An axis-aligned cube of side-length 2^(32-depth).
///
/// For a given side-length, the x,y,z values are indices into the grid of all such cubes.
///
/// In particular, x,y,z are numbers in the range from 0..2^depth.
#[derive(Debug, Clone, Copy, Default)]
struct BspCube {
    x: u32,
    y: u32,
    z: u32,
    /// 0..=32
    depth: u32,
}

#[derive(Debug, Clone, Copy)]
struct Bounds {
    /// Inclusive.
    min: Point,
    /// Inclusive.
    max: Point,
}

impl BspCube {
    fn bounds(self) -> Bounds {
        let mut x = self.x;
        let mut y = self.y;
        let mut z = self.z;

        let shift = 32 - self.depth;
        x <<= shift;
        y <<= shift;
        z <<= shift;
        let min = Point { x, y, z };

        let mask = 1_u32.unbounded_shl(shift).wrapping_sub(1);
        x += mask;
        y += mask;
        z += mask;
        let max = Point { x, y, z };

        Bounds { min, max }
    }

    /// Sub-divide into 8 cubes of half the side-length.
    fn split(mut self) -> Option<[Self; 8]> {
        if self.depth == 32 {
            return None;
        }

        self.depth += 1;
        self.x <<= 1;
        self.y <<= 1;
        self.z <<= 1;

        let mut out = [Self::default(); 8];
        for x in 0..2 {
            for y in 0..2 {
                for z in 0..2 {
                    out[4 * x + 2 * y + z] = self;
                    self.z |= 1;
                }
                self.y |= 1;
            }
            self.x |= 1;
        }
        Some(out)
    }

    fn intersects(self, s: Sphere) -> bool {
        let b = self.bounds();

        // Find the closest point to c.
        let c = s.center;
        let p = Point {
            x: c.x.clamp(b.min.x, b.max.x),
            y: c.y.clamp(b.min.y, b.max.y),
            z: c.z.clamp(b.min.z, b.max.z),
        };

        s.intersects(p)
    }
}

impl Sphere {
    fn intersects(self, p: Point) -> bool {
        let c = self.center;
        c.x.abs_diff(p.x) + c.y.abs_diff(p.y) + c.z.abs_diff(p.z) <= self.radius
    }
}

pub fn solve(nanobots: &[Nanobot]) -> usize {
    let spheres: Vec<_> = nanobots
        .iter()
        .map(|n| Sphere {
            center: n.pos.into(),
            radius: n.range.try_into().unwrap(),
        })
        .collect();

    let mut curr = BspCube::default();
    while let Some(sub_cubes) = curr.split() {
        curr = sub_cubes
            .into_iter()
            .max_by_key(|&cube| hit_count(cube, &spheres))
            .unwrap();
    }

    hit_count(curr, &spheres)
}

fn hit_count(cube: BspCube, spheres: &[Sphere]) -> usize {
    spheres.iter().filter(|&&s| cube.intersects(s)).count()
}
