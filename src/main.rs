use scanner::TokenType;
use std::env;
use std::fs;
use std::io;
use std::io::Write;

mod scanner;

fn run(source: &str) {
    let tokens = scanner::scan_tokens(source.to_string());
    println!("{:?}", tokens);
}

fn execute_repl() {
    loop {
        let mut input = String::new();
        io::stdout().write_all(b"> ").unwrap();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        run(&input);
    }
}

fn execute_file(file_path: &str) {
    run(&fs::read_to_string(file_path).unwrap());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: capers [script]");
        std::process::exit(64)
    }

    match args.get(1) {
        Some(file) => execute_file(file),
        None => execute_repl(),
    }
}
