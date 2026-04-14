use std::io::BufRead;

use anyhow::{Context, Result};
use itertools::Itertools;

pub type Input = Vec<Example>;

#[derive(Debug, Clone, Copy)]
pub struct Example {
    pub before: State,
    pub instruction: Instruction,
    pub after: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    pub registers: [u32; 4],
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub opcode: u32,
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

/*
parsing ideas

- slurp it and split on double-newline
    - use an empty stirng as the seciton separator

*/

pub fn parse<R: BufRead>(mut r: R) -> Result<Input> {
    let mut input = String::new();
    r.read_to_string(&mut input)?;

    let paragraphs = input.split("\n\n").collect_vec();
    let (examples, _) = paragraphs
        .split(|p| p.is_empty())
        .collect_tuple()
        .context("expected exactly one blank paragraph")?;

    let mut out = vec![];
    for example in examples {
        out.push(parse_example(example)?);
    }
    Ok(out)
}

fn parse_example(_lines: &str) -> Result<Example> {
    todo!()
}
