use std::io;

use day_17::input;

use anyhow::Result;

fn main() -> Result<()> {
    let input = input::parse(io::stdin().lock())?;
    println!("{}", day_17::part_1(input)?);
    Ok(())
}
