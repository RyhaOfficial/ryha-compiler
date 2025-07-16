// ryha-toolchain/ryha/src/main.rs

use ryha::codegen::CodeGenerator;
use ryha::ir::IRGenerator;
use ryha::lexer::Lexer;
use ryha::parser::Parser;
use ryha::semantic::SemanticAnalyzer;
use std::fs;
use std::io::{self, Read, Write};
use std::process::Command;
use tempfile::NamedTempFile;

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

    let mut asm_file = NamedTempFile::new().unwrap();
    asm_file.write_all(codegen.assembly().as_bytes()).unwrap();

    let obj_file = NamedTempFile::new().unwrap();

    let ryha_as_path = std::env::var("CARGO_BIN_EXE_ryha-as").unwrap();
    let as_output = Command::new(ryha_as_path)
        .arg(asm_file.path())
        .arg(obj_file.path())
        .output()
        .expect("Failed to execute ryha-as");

    if !as_output.status.success() {
        eprintln!("ryha-as failed:");
        eprintln!("{}", String::from_utf8_lossy(&as_output.stderr));
        return;
    }

    let output_file = "a.out";
    let ryha_ld_path = std::env::var("CARGO_BIN_EXE_ryha-ld").unwrap();
    let ld_output = Command::new(ryha_ld_path)
        .arg(obj_file.path())
        .arg(output_file)
        .output()
        .expect("Failed to execute ryha-ld");

    if !ld_output.status.success() {
        eprintln!("ryha-ld failed:");
        eprintln!("{}", String::from_utf8_lossy(&ld_output.stderr));
        return;
    }

    println!("Successfully compiled to {}", output_file);
}
