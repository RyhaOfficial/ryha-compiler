// ryha-toolchain/ryha/src/ir.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register(u8),
    Immediate(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Load(Operand, Operand),
    Store(Operand, Operand),
    Add(Operand, Operand, Operand),
}

pub struct IRGenerator {
    instructions: Vec<Instruction>,
    register_count: u8,
}

impl IRGenerator {
    pub fn new() -> Self {
        IRGenerator {
            instructions: Vec::new(),
            register_count: 0,
        }
    }

    fn next_register(&mut self) -> Operand {
        let reg = Operand::Register(self.register_count);
        self.register_count += 1;
        reg
    }

    pub fn generate(&mut self, program: &crate::ast::Program) {
        for statement in &program.statements {
            self.generate_statement(statement);
        }
    }

    fn generate_statement(&mut self, statement: &crate::ast::Statement) {
        match statement {
            crate::ast::Statement::Let(_, expr) => {
                self.generate_expression(expr);
            }
            _ => unimplemented!(),
        }
    }

    fn generate_expression(&mut self, expression: &crate::ast::Expression) -> Operand {
        match expression {
            crate::ast::Expression::IntLiteral(value) => {
                let reg = self.next_register();
                self.instructions.push(Instruction::Load(
                    reg.clone(),
                    Operand::Immediate(*value),
                ));
                reg
            }
            _ => unimplemented!(),
        }
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }
}
