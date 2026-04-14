use std::io;

use day_16::input;

use anyhow::Result;

fn main() -> Result<()> {
    let input = input::parse(io::stdin().lock())?;
    let ans = day_16::part_1(&input)?;
    println!("{}", ans);
    Ok(())
}
