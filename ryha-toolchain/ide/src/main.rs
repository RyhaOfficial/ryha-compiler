use rustyline::Editor;
use std::env;

fn main() -> rustyline::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--help" {
        println!("Ryha IDE");
        return Ok(());
    }

    let mut rl = Editor::<()>::new()?;
    loop {
        let readline = rl.readline(">> ")?;
        println!("Line: {}", readline);
    }
}
