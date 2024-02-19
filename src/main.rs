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
    let mut interpreter = Interpreter::default();
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(err) => {
            eprintln!("ERROR: Couldn't read file: {err}");
            return;
        }
    };
    run(&source, &mut interpreter);
}

fn run_prompt() {
    let mut interpreter = Interpreter::default();
    let mut iter = io::stdin().lines();
    loop {
        print!("> ");
        io::stdout().flush().expect("should flush stdout");
        let Some(Ok(line)) = iter.next() else {
            break;
        };
        run(&line, &mut interpreter);
    }
}

fn run(source: &str, interpreter: &mut Interpreter) {
    let tokens = Scanner::scan_tokens(source);
    let Some(statements) = Parser::parse(tokens) else {
        return;
    };
    interpreter.interpret(&statements);
}
