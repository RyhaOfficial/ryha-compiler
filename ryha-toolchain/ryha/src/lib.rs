// ryha-toolchain/ryha/src/lib.rs

pub mod ast;
pub mod codegen;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod semantic;

#[cfg(test)]
mod tests {
    use super::lexer::Lexer;
    use super::parser::Parser;
    use super::ast::Statement;

    #[test]
    fn test_let_statement() {
        let input = "let x = 5;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        let stmt = &program.statements[0];
        if let Statement::Let(name, _) = stmt {
            assert_eq!(name, "x");
        } else {
            panic!("Expected Let statement");
        }
    }

    #[test]
    fn test_undeclared_variable() {
        let input = "x;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let mut analyzer = crate::semantic::SemanticAnalyzer::new();
        analyzer.analyze(&program);

        assert_eq!(analyzer.errors().len(), 1);
        assert_eq!(analyzer.errors()[0], "undefined variable: x");
    }

    #[test]
    fn test_ir_generation() {
        let input = "let x = 5;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let mut ir_generator = crate::ir::IRGenerator::new();
        ir_generator.generate(&program);

        let instructions = ir_generator.instructions();
        assert_eq!(instructions.len(), 1);
        assert_eq!(
            instructions[0],
            crate::ir::Instruction::Load(
                crate::ir::Operand::Register(0),
                crate::ir::Operand::Immediate(5)
            )
        );
    }

    #[test]
    fn test_codegen() {
        let input = "let x = 5;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let mut ir_generator = crate::ir::IRGenerator::new();
        ir_generator.generate(&program);

        let mut codegen = crate::codegen::CodeGenerator::new();
        codegen.generate(ir_generator.instructions());

        assert_eq!(codegen.assembly(), ".global main\nmain:\nmov r0, 5\n");
    }

    #[test]
    fn test_infix_expression() {
        let input = "let x = 5 + 10;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let mut ir_generator = crate::ir::IRGenerator::new();
        ir_generator.generate(&program);

        let mut codegen = crate::codegen::CodeGenerator::new();
        codegen.generate(ir_generator.instructions());

        assert_eq!(
            codegen.assembly(),
            ".global main\nmain:\nmov r0, 5\nmov r1, 10\nmov r2, r0\nadd r2, r1\n"
        );
    }

    #[test]
    fn test_return_statement() {
        let input = "return 5;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let mut ir_generator = crate::ir::IRGenerator::new();
        ir_generator.generate(&program);

        let mut codegen = crate::codegen::CodeGenerator::new();
        codegen.generate(ir_generator.instructions());

        assert_eq!(
            codegen.assembly(),
            ".global main\nmain:\nmov r0, 5\nmov rax, r0\nret\n"
        );
    }
}
