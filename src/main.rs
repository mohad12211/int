use std::{
    env,
    fs::{self},
    io::{self, Write},
    process::exit,
};

use int::interpreter::Interpreter;

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
    interpreter.interpret(source);
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
        interpreter.interpret(line);
    }
}
