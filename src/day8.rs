use std::io::Error;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashSet;

use crate::utility::{partition, parse_file_lines};


pub fn solve() -> Result<(), Error> {
    let instructions: Vec<Instruction> = parse_file_lines("data/day8.txt")?;
    let (total, _) = interpret_once(&instructions);
    println!("Default Acc {}", total);
    for (index, instruction) in instructions.iter().enumerate().rev() {
        match instruction.operation {
            Operation::JMP | Operation::NOP => {
                match interpret_once(&swap_instruction(&instructions, index)) {
                    (total, true) => {
                        println!("Fixed Acc {} @ {}", total, index);
                        break}
                    ,
                    _ => (),
                }
            },
            Operation::ACC => (),
        }
    }
    Ok(())
}

#[derive(Debug)]
#[derive(Clone)]
enum Operation {NOP, ACC, JMP}

#[derive(Debug)]
#[derive(Clone)]
struct Instruction {
    operation: Operation,
    argument: isize,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    /// Read a literal such as  `jmp +4`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, arg) = partition(&s, " ");
        let operation = match op.as_str() {
            "acc" => Operation::ACC,
            "jmp" => Operation::JMP,
            _ => Operation::NOP,
        };
        let argument = match arg {
            Some(x) => x.parse::<isize>()?,
            _ => 0,
        };
        Ok(Instruction {operation, argument})
    }
}

/// Run `instructions` until hitting a loop or terminating
/// Return the accumulated value and whether termination was proper.
fn interpret_once(instructions: &Vec<Instruction>) -> (isize, bool) {
    let mut pointer: isize = 0;
    let mut accumulator = 0;
    let mut seen = HashSet::new();
    while !seen.contains(&pointer) && (pointer as usize) < instructions.len() {
        seen.insert(pointer.clone());
        match &instructions[pointer as usize] {
            Instruction {operation: Operation::JMP, argument: x} => pointer += x,
            Instruction {operation: Operation::ACC, argument: x} => {
                accumulator += x; pointer += 1;
            },
            Instruction {operation: Operation::NOP, argument: _} => pointer += 1,
        }
    }
    (accumulator, (pointer as usize) >= instructions.len())
}

/// Generate new `instructions` by swapping a JMP/NOP instruction at `index`
fn swap_instruction(instructions: &Vec<Instruction>, index: usize) -> Vec<Instruction> {
    let mut new_instructions = instructions.clone();
    match new_instructions[index] {
        Instruction {operation: Operation::ACC, argument: _} => {
            panic!("Can only swap JMP and NOP operations!");
        },
        Instruction {operation: Operation::JMP, argument} => {
            new_instructions[index] = Instruction {operation: Operation::NOP, argument}
        },
        Instruction {operation: Operation::NOP, argument} => {
            new_instructions[index] = Instruction {operation: Operation::JMP, argument}
        },
    };
    new_instructions
}