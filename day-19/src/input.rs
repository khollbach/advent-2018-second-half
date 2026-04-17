use std::io::BufRead;

use anyhow::{Context, Result, bail};
use itertools::Itertools;

use crate::program::{Instruction, Operation};

pub fn parse<R: BufRead>(r: R) -> Result<(usize, Vec<Instruction>)> {
    let mut lines = r.lines();

    let ip: usize = parse_ip(&lines.next().context("empty input")??)?;

    let program = lines.map(|l| parse_instruction(&l?)).try_collect()?;

    Ok((ip, program))
}

fn parse_ip(line: &str) -> Result<usize> {
    let ip = line
        .strip_prefix("#ip ")
        .context("expected `#ip `")?
        .parse()?;
    Ok(ip)
}

fn parse_instruction(line: &str) -> Result<Instruction> {
    let (op, a, b, c) = line
        .split_whitespace()
        .collect_tuple()
        .context("expected 4 words")?;
    Ok(Instruction {
        op: parse_op(op)?,
        a: a.parse()?,
        b: b.parse()?,
        c: c.parse()?,
    })
}

fn parse_op(op: &str) -> Result<Operation> {
    Ok(match op {
        "addr" => Operation::Addr,
        "addi" => Operation::Addi,
        "mulr" => Operation::Mulr,
        "muli" => Operation::Muli,
        "banr" => Operation::Banr,
        "bani" => Operation::Bani,
        "borr" => Operation::Borr,
        "bori" => Operation::Bori,
        "setr" => Operation::Setr,
        "seti" => Operation::Seti,
        "gtir" => Operation::Gtir,
        "gtri" => Operation::Gtri,
        "gtrr" => Operation::Gtrr,
        "eqir" => Operation::Eqir,
        "eqri" => Operation::Eqri,
        "eqrr" => Operation::Eqrr,
        _ => bail!("invalid op: {op:?}"),
    })
}
