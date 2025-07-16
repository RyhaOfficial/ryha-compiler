// ryha-toolchain/ryha/src/obfuscator.rs

use crate::ir::Instruction;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Obfuscator {
    instructions: Vec<Instruction>,
}

impl Obfuscator {
    pub fn new() -> Self {
        Obfuscator {
            instructions: Vec::new(),
        }
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn obfuscate(&mut self, instructions: &mut [Instruction]) {
        instructions.shuffle(&mut thread_rng());
        self.instructions = instructions.to_vec();
    }
}
