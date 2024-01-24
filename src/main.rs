use std::{
    env,
    fs::{self},
    io::{self, Write},
    process::exit,
};

use int::{interpreter::Interpreter, parser::Parser, scanner::Scanner};

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
    let mut interpreter = Interpreter::new();
    let source = fs::read_to_string(path).expect("should read file");
    run(source, &mut interpreter);
}

fn run_prompt() {
    let mut interpreter = Interpreter::new();
    let mut iter = io::stdin().lines();
    loop {
        print!("> ");
        io::stdout().flush().expect("should flush stdout");
        let Some(Ok(line)) = iter.next() else {
            break;
        };
        run(line, &mut interpreter);
    }
}

fn run(source: String, interpreter: &mut Interpreter) {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();
    let mut parser = Parser::new(scanner.tokens);
    parser.parse();
    interpreter.interpret(parser.statements);
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
