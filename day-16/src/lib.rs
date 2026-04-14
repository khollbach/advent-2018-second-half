/*
todos:
- input parsing

*/

use crate::input::{Input, Instruction};
use anyhow::Result;

pub mod input;
mod operation;

pub fn part_1(input: &Input) -> Result<usize> {
    let mut num_interesting_examples = 0;

    for example in input {
        let Instruction { opcode: _, a, b, c } = example.instruction;

        let mut num_matching_operations = 0;
        for op in operation::OPERATIONS {
            if op.apply(example.before, a, b, c)? == example.after {
                num_matching_operations += 1;
            }
        }
        if num_matching_operations >= 3 {
            num_interesting_examples += 1;
        }
    }

    Ok(num_interesting_examples)
}
