use std::io;

use day_16::input;

use anyhow::Result;

fn main() -> Result<()> {
    let (examples, program) = input::parse(io::stdin().lock())?;
    println!("{}", day_16::part_1(&examples)?);
    println!("{}", day_16::part_2(&examples, &program)?);
    Ok(())
}
