use std::{array, collections::HashSet};

use crate::{
    input::{Example, Instruction, State},
    operation::{NUM_OPS, OPERATIONS, Operation},
};
use anyhow::{Context, Result, ensure};
use itertools::Itertools;

pub mod input;
mod operation;

pub fn part_1(examples: &[Example]) -> Result<usize> {
    let mut num_interesting_examples = 0;

    for example in examples {
        let Instruction { opcode: _, a, b, c } = example.instruction;

        let mut num_matching_operations = 0;
        for op in OPERATIONS {
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

pub fn part_2(examples: &[Example], program: &[Instruction]) -> Result<u32> {
    let possible_operations = possible_operations(examples)?;
    let mapping = solve_constraints(possible_operations)?;
    run_program(mapping, program)
}

/// For each opcode, what operations could it be?
fn possible_operations(examples: &[Example]) -> Result<[HashSet<Operation>; NUM_OPS]> {
    let all_operations: HashSet<_> = OPERATIONS.into_iter().collect();

    let mut possible_operations: [_; NUM_OPS] = array::repeat(all_operations);

    for example in examples {
        let Instruction { opcode, a, b, c } = example.instruction;
        let opcode = usize::try_from(opcode).unwrap();
        ensure!(opcode < NUM_OPS);

        for op in OPERATIONS {
            if op.apply(example.before, a, b, c)? != example.after {
                possible_operations[opcode].remove(&op);
            }
        }
    }

    Ok(possible_operations)
}

fn solve_constraints(
    mut possible_operations: [HashSet<Operation>; NUM_OPS],
) -> Result<[Operation; NUM_OPS]> {
    let mut mapping = [Operation::Addi; NUM_OPS];

    while possible_operations.iter().any(|set| !set.is_empty()) {
        // Find an opcode with only one candidate.
        let (opcode, set) = possible_operations
            .iter_mut()
            .find_position(|set| set.len() == 1)
            .context("can't solve constraints")?;
        let op = *set.iter().next().unwrap();

        mapping[opcode] = op;

        // Remove `op` from all sets, including this one.
        for set in &mut possible_operations {
            set.remove(&op);
        }
    }

    Ok(mapping)
}

fn run_program(mapping: [Operation; NUM_OPS], program: &[Instruction]) -> Result<u32> {
    let mut state = State { registers: [0; 4] };

    for &Instruction { opcode, a, b, c } in program {
        let opcode = usize::try_from(opcode).unwrap();
        ensure!(opcode < NUM_OPS);
        let op = mapping[opcode];

        state = op.apply(state, a, b, c)?;
    }

    Ok(state.registers[0])
}
