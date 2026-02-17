use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: prizm <command> <file>");
        return;
    }

    let command = &args[1];
    let file = &args[2];

    match command.as_str() {
        "run" => println!("Running {}...", file),
        "pretty" => println!("Formatting {}...", file),
        "lint" => println!("Linting {}...", file),
        _ => eprintln!("Unknown command: {}", command),
    }
}