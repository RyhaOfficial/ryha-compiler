// ryha-toolchain/ryha/src/codegen.rs

pub struct CodeGenerator {
    assembly: String,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            assembly: String::new(),
        }
    }

    pub fn assembly(&self) -> &str {
        &self.assembly
    }

    pub fn generate(&mut self, instructions: &[crate::ir::Instruction]) {
        for instruction in instructions {
            self.generate_instruction(instruction);
        }
    }

    fn generate_instruction(&mut self, instruction: &crate::ir::Instruction) {
        match instruction {
            crate::ir::Instruction::Load(dest, src) => {
                let dest_reg = self.operand_to_reg(dest);
                let src_val = self.operand_to_val(src);
                self.emit(format!("mov {}, {}", dest_reg, src_val));
            }
            _ => unimplemented!(),
        }
    }

    fn operand_to_reg(&self, operand: &crate::ir::Operand) -> String {
        match operand {
            crate::ir::Operand::Register(r) => format!("r{}", r),
            _ => unimplemented!(),
        }
    }

    fn operand_to_val(&self, operand: &crate::ir::Operand) -> String {
        match operand {
            crate::ir::Operand::Immediate(i) => i.to_string(),
            _ => unimplemented!(),
        }
    }

    fn emit(&mut self, s: String) {
        self.assembly.push_str(&s);
        self.assembly.push('\n');
    }
}
