use std::{
    env,
    fs::{self},
    io::{self, Write},
    process::exit,
};

mod scanner;
mod token;

use crate::scanner::Scanner;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: {} [script]", args[0]);
            exit(1);
        }
    };
}
fn run_file(path: &str) {
    let source = fs::read_to_string(path).expect("Unable to read file");
    run(source);
}

fn run_prompt() {
    let mut iter = io::stdin().lines();
    loop {
        print!("> ");
        io::stdout().flush().expect("flush stdout");
        let Some(Ok(line)) = iter.next() else {
            break;
        };
        run(line);
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{token:?}");
    }
}

// TODO: add error handling
//
// fn error(line: usize, message: &str) {
//     report(line, "", message);
// }
//
// fn report(line: usize, location: &str, message: &str) {
//     println!("[line {line}] Error{location}: {message}");
// }
