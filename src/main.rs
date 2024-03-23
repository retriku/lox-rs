#[macro_use]
extern crate text_io;

use std::env;
use std::fs;

use scanner::Scanner;

mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: &[String] = &args.as_slice()[1..];
    if args.len() > 1 {
        println!("Usage: lox-rs [script]");
        std::process::exit(exitcode::USAGE);
    } else if args.len() == 1 {
        run_file(args[0].as_str());
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) -> () {
    let code: String = fs::read_to_string(path).unwrap().parse().unwrap();
    run(code);
}

fn run(source: String) -> () {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}

fn run_prompt() -> () {
    'eof: loop {
        print!("> ");
        let line: String = read!("{}\n");
        if line.len() == 0 {
            break 'eof;
        }
        run(line);
    }
}
