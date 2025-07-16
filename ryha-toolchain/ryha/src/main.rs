// ryha-toolchain/ryha/src/main.rs

use ryha::codegen::CodeGenerator;
use ryha::ir::IRGenerator;
use ryha::lexer::Lexer;
use ryha::parser::Parser;
use ryha::semantic::SemanticAnalyzer;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("Failed to read from stdin");
        return;
    }

    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    if !parser.errors().is_empty() {
        eprintln!("Parser errors:");
        for error in parser.errors() {
            eprintln!("\t{}", error);
        }
        return;
    }

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&program);

    if !analyzer.errors().is_empty() {
        eprintln!("Semantic errors:");
        for error in analyzer.errors() {
            eprintln!("\t{}", error);
        }
        return;
    }

    let mut ir_generator = IRGenerator::new();
    ir_generator.generate(&program);

    let mut codegen = CodeGenerator::new();
    codegen.generate(ir_generator.instructions());

    println!("{}", codegen.assembly());
}
