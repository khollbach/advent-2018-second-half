use crate::input::{Example, Instruction};
use anyhow::Result;

pub mod input;
mod operation;

pub fn part_1(examples: &[Example]) -> Result<usize> {
    let mut num_interesting_examples = 0;

    for example in examples {
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

/*
todo: pcode

*/

pub fn part_2(_examples: &[Example], _program: &[Instruction]) -> Result<usize> {
    todo!()
}
