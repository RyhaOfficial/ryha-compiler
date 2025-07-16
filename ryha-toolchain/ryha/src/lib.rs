// ryha-toolchain/ryha/src/lib.rs

pub mod ast;
pub mod codegen;
pub mod gemini;
pub mod ir;
pub mod lexer;
pub mod obfuscator;
pub mod optimizer;
pub mod parser;
pub mod self_modifier;
pub mod semantic;
pub mod voice;

use std::io::Write;

#[cfg(test)]
mod tests {
    use super::lexer::Lexer;
    use super::parser::Parser;
    use super::ast::Statement;
    use std::io::Write;

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

    #[test]
    fn test_end_to_end() {
        if std::process::Command::new("nasm").output().is_err() {
            eprintln!("nasm not found, skipping end-to-end test");
            return;
        }

        // Build the ryha-as and ryha-ld executables
        let build_output = std::process::Command::new("cargo")
            .arg("build")
            .arg("--package")
            .arg("ryha-as")
            .arg("--package")
            .arg("ryha-ld")
            .output()
            .expect("Failed to build ryha-as and ryha-ld");

        assert!(build_output.status.success());

        let input = "return 42;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let mut ir_generator = crate::ir::IRGenerator::new();
        ir_generator.generate(&program);

        let mut codegen = crate::codegen::CodeGenerator::new();
        codegen.generate(ir_generator.instructions());

        let mut asm_file = tempfile::NamedTempFile::new().unwrap();
        asm_file.write_all(codegen.assembly().as_bytes()).unwrap();

        let obj_file = tempfile::NamedTempFile::new().unwrap();

        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let as_output = std::process::Command::new("cargo")
            .arg("run")
            .arg("--package")
            .arg("ryha-as")
            .arg("--")
            .arg(asm_file.path())
            .arg(obj_file.path())
            .output()
            .expect("Failed to execute ryha-as");

        if !as_output.status.success() {
            panic!(
                "ryha-as failed: {}",
                String::from_utf8_lossy(&as_output.stderr)
            );
        }

        let output_file = "test_end_to_end";
        let ld_output = std::process::Command::new("cargo")
            .arg("run")
            .arg("--package")
            .arg("ryha-ld")
            .arg("--")
            .arg(obj_file.path())
            .arg(output_file)
            .output()
            .expect("Failed to execute ryha-ld");

        assert!(ld_output.status.success());

        let run_output = std::process::Command::new(format!("./{}", output_file))
            .output()
            .expect("Failed to run executable");

        assert_eq!(run_output.status.code(), Some(42));
    }

    #[test]
    fn test_constant_folding() {
        let input = "let x = 5 + 10;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let mut ir_generator = crate::ir::IRGenerator::new();
        ir_generator.generate(&program);

        let mut optimizer = crate::optimizer::Optimizer::new();
        optimizer.optimize(ir_generator.instructions());

        let instructions = optimizer.instructions();
        assert_eq!(instructions.len(), 1);
        assert_eq!(
            instructions[0],
            crate::ir::Instruction::Load(
                crate::ir::Operand::Register(2),
                crate::ir::Operand::Immediate(15)
            )
        );
    }

    #[test]
    fn test_voice_command_parser() {
        let parser = crate::voice::VoiceCommandParser::new();
        assert_eq!(parser.parse("build"), Some("build".to_string()));
        assert_eq!(parser.parse("test"), None);
    }

    #[tokio::test]
    async fn test_gemini_client() {
        let client = crate::gemini::GeminiClient::new("test_api_key".to_string());
        let explanation = client.explain("test error").await;
        assert!(explanation.is_err());
    }

    #[test]
    fn test_self_modifier() {
        let self_modifier = crate::self_modifier::SelfModifier::new();
        self_modifier.improve_security();
        let main_rs = std::fs::read_to_string("src/main.rs").unwrap();
        assert!(main_rs.contains("with enhanced security!"));
    }

    #[test]
    fn test_obfuscator() {
        let input = "let x = 5 + 10;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let mut ir_generator = crate::ir::IRGenerator::new();
        ir_generator.generate(&program);

        let mut optimizer = crate::optimizer::Optimizer::new();
        optimizer.optimize(ir_generator.instructions());

        let mut obfuscator = crate::obfuscator::Obfuscator::new();
        obfuscator.obfuscate(&mut optimizer.instructions().to_vec());

        let instructions = obfuscator.instructions();
        assert_eq!(instructions.len(), 1);
    }
}
