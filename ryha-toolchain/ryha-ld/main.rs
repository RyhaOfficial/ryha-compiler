use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.o> <output>", args[0]);
        return;
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let ld_output = Command::new("ld")
        .arg(input_file)
        .arg("-o")
        .arg(output_file)
        .output()
        .expect("Failed to execute ld");

    if !ld_output.status.success() {
        eprintln!("ld failed:");
        eprintln!("{}", String::from_utf8_lossy(&ld_output.stderr));
    }
}
