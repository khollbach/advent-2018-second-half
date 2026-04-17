use std::io;

use anyhow::Result;

mod input;
mod program;

fn main() -> Result<()> {
    let (ip, code) = input::parse(io::stdin().lock())?;
    println!("{}", program::run(ip, code.clone(), 0)?);
    // println!("{}", program::run(ip, code, 1)?);
    Ok(())
}
