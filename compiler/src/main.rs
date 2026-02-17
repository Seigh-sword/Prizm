mod attributes;
mod lexer;
mod stdlib;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: prizm <command> <file>");
        eprintln!("Commands: run, pretty, lint");
        return;
    }

    let command = &args[1];
    let file = &args[2];

    match command.as_str() {
        "run" => run_file(file),
        "pretty" => pretty_file(file),
        "lint" => lint_file(file),
        _ => eprintln!("Unknown command: {}", command),
    }
}

fn run_file(file: &str) {
    match std::fs::read_to_string(file) {
        Ok(content) => {
            println!("Running {}...", file);
            let mut lexer = lexer::Lexer::new(&content);
            let tokens = lexer.tokenize();
            println!("Tokens: {:?}", tokens);
            // TODO: Parse and execute tokens
        }
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}

fn pretty_file(file: &str) {
    match std::fs::read_to_string(file) {
        Ok(content) => {
            println!("Formatting {}...", file);
            let mut lexer = lexer::Lexer::new(&content);
            let tokens = lexer.tokenize();
            // TODO: Pretty print tokens
            println!("Tokens: {:?}", tokens);
        }
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}

fn lint_file(file: &str) {
    match std::fs::read_to_string(file) {
        Ok(content) => {
            println!("Linting {}...", file);
            let mut lexer = lexer::Lexer::new(&content);
            let tokens = lexer.tokenize();
            // TODO: Perform syntax and semantic checks
            println!("Tokens: {:?}", tokens);
        }
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}