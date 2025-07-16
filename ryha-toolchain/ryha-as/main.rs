use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.asm> <output.o>", args[0]);
        return;
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let nasm_output = Command::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg(input_file)
        .arg("-o")
        .arg(output_file)
        .output()
        .expect("Failed to execute nasm");

    if !nasm_output.status.success() {
        eprintln!("nasm failed:");
        eprintln!("{}", String::from_utf8_lossy(&nasm_output.stderr));
    }
}
