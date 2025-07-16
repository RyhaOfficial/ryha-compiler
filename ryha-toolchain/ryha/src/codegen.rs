// ryha-toolchain/ryha/src/codegen.rs

use std::collections::HashSet;

pub struct CodeGenerator {
    assembly: String,
    used_registers: HashSet<u8>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            assembly: String::new(),
            used_registers: HashSet::new(),
        }
    }

    pub fn assembly(&self) -> &str {
        &self.assembly
    }

    pub fn generate(&mut self, instructions: &[crate::ir::Instruction]) {
        self.emit(".global main".to_string());
        self.emit("main:".to_string());

        let mut used_regs: Vec<u8> = self.used_registers.iter().cloned().collect();
        used_regs.sort();

        for reg in &used_regs {
            self.emit(format!("push r{}", 12 + reg));
        }

        for instruction in instructions {
            self.generate_instruction(instruction);
        }

        for reg in used_regs.iter().rev() {
            self.emit(format!("pop r{}", 12 + reg));
        }
    }

    fn generate_instruction(&mut self, instruction: &crate::ir::Instruction) {
        match instruction {
            crate::ir::Instruction::Load(dest, src) => {
                let dest_reg = self.operand_to_reg(dest);
                let src_val = self.operand_to_val(src);
                self.emit(format!("mov {}, {}", dest_reg, src_val));
            }
            crate::ir::Instruction::Add(dest, src1, src2) => {
                let dest_reg = self.operand_to_reg(dest);
                let src1_reg = self.operand_to_reg(src1);
                let src2_reg = self.operand_to_reg(src2);
                self.emit(format!("mov {}, {}", dest_reg, src1_reg));
                self.emit(format!("add {}, {}", dest_reg, src2_reg));
            }
            crate::ir::Instruction::Ret(reg) => {
                let ret_reg = self.operand_to_reg(reg);
                self.emit(format!("mov rax, {}", ret_reg));
                self.emit("ret".to_string());
            }
            crate::ir::Instruction::Call(name, args) => {
                for (i, arg) in args.iter().enumerate() {
                    let arg_reg = self.operand_to_reg(arg);
                    let param_reg = format!("rdi{}", i);
                    self.emit(format!("mov {}, {}", param_reg, arg_reg));
                }
                let func_name = self.operand_to_val(name);
                self.emit(format!("call {}", func_name));
            }
            _ => unimplemented!(),
        }
    }

    fn operand_to_reg(&mut self, operand: &crate::ir::Operand) -> String {
        match operand {
            crate::ir::Operand::Register(r) => {
                self.used_registers.insert(*r);
                format!("r{}", r)
            }
            _ => unimplemented!(),
        }
    }

    fn operand_to_val(&self, operand: &crate::ir::Operand) -> String {
        match operand {
            crate::ir::Operand::Immediate(i) => i.to_string(),
            crate::ir::Operand::Label(l) => l.clone(),
            _ => unimplemented!(),
        }
    }

    fn emit(&mut self, s: String) {
        self.assembly.push_str(&s);
        self.assembly.push('\n');
    }
}
