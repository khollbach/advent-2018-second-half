use std::io::BufRead;
use std::str::FromStr;

use anyhow::{Context, Result, anyhow};
use itertools::Itertools;

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

pub fn parse<R: BufRead>(mut r: R) -> Result<(Vec<Example>, Vec<Instruction>)> {
    let mut everything = String::new();
    r.read_to_string(&mut everything)?;

    let paragraphs = everything.split("\n\n").collect_vec();
    let (examples, program) = paragraphs
        .split(|p| p.is_empty())
        .collect_tuple()
        .context("expected exactly one empty paragraph")?;

    let mut parsed_examples = vec![];
    for example in examples {
        parsed_examples.push(parse_example(example)?);
    }

    let mut instructions = vec![];
    for line in program {
        instructions.push(parse_nums(line)?.into());
    }

    Ok((parsed_examples, instructions))
}

fn parse_example(lines: &str) -> Result<Example> {
    let (before, instruction, after) = lines
        .split('\n')
        .collect_tuple()
        .context("expected 3 lines")?;

    let list = before
        .strip_prefix("Before: ")
        .context("expected `Before: `")?;
    let before = parse_list(list)?;

    let instruction = parse_nums(instruction)?;

    let list = after
        .strip_prefix("After:  ") // note the double space
        .context("expected `After:  `")?;
    let after = parse_list(list)?;

    Ok(Example {
        before: before.into(),
        instruction: instruction.into(),
        after: after.into(),
    })
}

fn parse_list(s: &str) -> Result<[u32; 4]> {
    let list: Vec<_> = s
        .strip_prefix('[')
        .context("expected [")?
        .strip_suffix(']')
        .context("expected ]")?
        .split(", ")
        .map(u32::from_str)
        .try_collect()?;
    list.try_into()
        .map_err(|_| anyhow!("expected exactly 4 nums"))
}

fn parse_nums(s: &str) -> Result<[u32; 4]> {
    let list: Vec<u32> = s.split(' ').map(u32::from_str).try_collect()?;
    list.try_into()
        .map_err(|_| anyhow!("expected exactly 4 nums"))
}

impl From<[u32; 4]> for State {
    fn from(registers: [u32; 4]) -> Self {
        Self { registers }
    }
}

impl From<[u32; 4]> for Instruction {
    fn from([opcode, a, b, c]: [u32; 4]) -> Self {
        Self { opcode, a, b, c }
    }
}
