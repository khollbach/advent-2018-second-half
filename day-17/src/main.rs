use std::io;

use anyhow::Result;
use day_17::{grid::Point, input};

// The code for today is very messy ... be warned!

fn main() -> Result<()> {
    let mut grid = input::parse(io::stdin().lock())?;

    // println!("{}", day_17::part_1(input)?);

    grid.print();
    grid.pour_water(Point { x: 500, y: 0 })?;
    println!();
    println!();
    println!();
    grid.print();

    // 31606 -- "wrong, too low", and took quite a while (30+ seconds maybe -- I wasn't looking)
    // the printouts look very reasonable though...
    // (maybe add wetness to the printout??)

    println!("{}", grid.count_wet_part_1());
    println!("{}", grid.count_water_part_2());

    Ok(())
}
