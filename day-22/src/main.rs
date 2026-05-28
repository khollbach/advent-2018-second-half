fn main() {
    let depth = 11991;
    let target = (6, 797);

    let grid = create_grid(depth, target);
    print_grid(&grid);
    println!("{}", risk_level(&grid, target));
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

fn create_grid(depth: u32, target: (usize, usize)) -> Vec<Vec<u32>> {
    let mod_ = 20_183;

    let (x, y) = target;
    let num_rows = y + 1;
    let num_cols = y + 1;

    // Storing erosion level (mod mod_).
    let mut grid = vec![vec![0; num_cols]; num_rows];

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
            grid[y][x] = grid[y - 1][x] * grid[y][x - 1] % mod_;
            grid[y][x] += depth;
            grid[y][x] %= mod_;
        }
    }

    // Special case: target.
    grid[y][x] = 0;
    grid[y][x] += depth;
    grid[y][x] %= mod_;

    grid
}

fn print_grid(grid: &Vec<Vec<u32>>) {
    for row in grid {
        for x in row {
            let c = match x % 3 {
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

fn risk_level(grid: &Vec<Vec<u32>>, target: (usize, usize)) -> u32 {
    let mut out = 0;

    let (x, y) = target;
    for y in 0..=y {
        for x in 0..=x {
            out += grid[y][x] % 3;
        }
    }

    out
}
