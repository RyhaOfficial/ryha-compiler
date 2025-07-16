// ryha-toolchain/ryha/src/security.rs

use crate::ir::Instruction;

pub struct SecurityAnalyzer {
    errors: Vec<String>,
}

impl SecurityAnalyzer {
    pub fn new() -> Self {
        SecurityAnalyzer { errors: Vec::new() }
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    pub fn analyze(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction {
                Instruction::Store(dest, _) => {
                    // This is a very simple check. A real implementation would need to
                    // perform more sophisticated analysis to determine the size of the
                    // destination buffer and the size of the data being stored.
                    self.errors
                        .push(format!("potential buffer overflow at {:?}", dest));
                }
                _ => {}
            }
        }
    }
}
