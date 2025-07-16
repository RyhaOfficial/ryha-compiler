// ryha-toolchain/ryha/src/optimizer.rs

use crate::ir::{Instruction, Operand};

pub struct Optimizer {
    instructions: Vec<Instruction>,
}

impl Optimizer {
    pub fn new() -> Self {
        Optimizer {
            instructions: Vec::new(),
        }
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn optimize(&mut self, instructions: &[Instruction]) {
        let mut new_instructions = Vec::new();

        for (i, instruction) in instructions.iter().enumerate() {
            match instruction {
                Instruction::Add(dest, src1, src2) => {
                    if i > 1 {
                        if let (Some(i1), Some(i2)) = (
                            self.is_immediate(&instructions[i - 2]),
                            self.is_immediate(&instructions[i - 1]),
                        ) {
                            new_instructions.pop();
                            new_instructions.pop();
                            new_instructions.push(Instruction::Load(
                                dest.clone(),
                                Operand::Immediate(i1 + i2),
                            ));
                        } else {
                            new_instructions.push(instruction.clone());
                        }
                    } else {
                        new_instructions.push(instruction.clone());
                    }
                }
                _ => new_instructions.push(instruction.clone()),
            }
        }

        self.instructions = new_instructions;
    }

    fn is_immediate(&self, instruction: &Instruction) -> Option<i64> {
        if let Instruction::Load(_, Operand::Immediate(i)) = instruction {
            Some(*i)
        } else {
            None
        }
    }
}
