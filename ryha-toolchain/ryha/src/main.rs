// ryha-toolchain/ryha/src/main.rs

use ryha::codegen::CodeGenerator;
use ryha::gemini::GeminiClient;
use ryha::ir::IRGenerator;
use ryha::lexer::Lexer;
use ryha::obfuscator::Obfuscator;
use ryha::optimizer::Optimizer;
use ryha::parser::Parser;
use ryha::security::SecurityAnalyzer;
use ryha::self_modifier::SelfModifier;
use ryha::semantic::SemanticAnalyzer;
use ryha::voice::VoiceCommandParser;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::process::Command;
use tempfile::NamedTempFile;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--voice" {
        let voice_command_parser = VoiceCommandParser::new();
        let command = voice_command_parser.parse(&args[2]);
        if command == Some("build".to_string()) {
            build(false, false);
        } else if command == Some("improve security".to_string()) {
            let self_modifier = SelfModifier::new();
            self_modifier.improve_security();
            println!("Security improved!");
        } else {
            eprintln!("Invalid voice command");
        }
    } else if args.len() > 1 && args[1] == "--explain" {
        let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
        let client = GeminiClient::new(api_key);
        let error = &args[2];
        let explanation = client.explain(error).await.unwrap();
        println!("{}", explanation);
    } else if args.len() > 1 && args[1] == "--ide" {
        let ide_path = std::env::var("CARGO_BIN_EXE_ide").unwrap();
        Command::new(ide_path).status().unwrap();
    } else if args.len() > 1 && args[1] == "--obfuscate" {
        build(true, false);
    } else if args.len() > 1 && args[1] == "--secure-mode" {
        build(false, true);
    } else {
        build(false, false);
    }
}

fn build(obfuscate: bool, secure_mode: bool) {
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

    let mut optimizer = Optimizer::new();
    optimizer.optimize(ir_generator.instructions());

    let instructions = if obfuscate {
        let mut obfuscator = Obfuscator::new();
        obfuscator.obfuscate(&mut optimizer.instructions().to_vec());
        obfuscator.instructions().to_vec()
    } else {
        optimizer.instructions().to_vec()
    };

    let mut codegen = CodeGenerator::new();
    codegen.generate(&instructions);

    if secure_mode {
        let mut security_analyzer = SecurityAnalyzer::new();
        security_analyzer.analyze(&instructions);
        if !security_analyzer.errors().is_empty() {
            eprintln!("Security errors:");
            for error in security_analyzer.errors() {
                eprintln!("\t{}", error);
            }
            return;
        }
    }

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

    println!("Successfully compiled to {} with enhanced security!", output_file);
}
