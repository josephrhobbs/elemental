//! Main executable for the Elemental language interpreter.

use std::{
    io::{
        self,
        Write,
    },
    env,
    fs,
    process::exit,
    collections::HashMap,
};

use colored::*;

use elemental::interpret;
use elemental::error::*;

const VERSION: &str = "0.5.0";

fn main() {
    if env::args().len() < 2 {
        interpreter();
    }

    // Get the input file
    let input_file: String = match env::args().nth(1) {
        Some(f) => f.to_owned(),
        None => unreachable!(), // This code is unreachable as we know the length of `env::args()` is at least 2
    };

    let code: Vec<String> = match fs::read_to_string(input_file.to_owned()) {
        Ok(c) => c.split("\n").map(|x| x.to_string()).collect::<Vec<String>>(),
        Err(_) => {
            throw(CouldNotReadFile (input_file));
            exit(0);
        },
    };

    // Store a list of variables in the program
    let mut variables = HashMap::new();

    for mut command in code {
        // For the tokenizer to work, we add `\n` to each line
        command.push('\n');

        let (expression, is_silent) = interpret(&mut variables, command.to_owned());

        // Only if it is not "silent", print the input and output
        if !is_silent {
            let output = format!(
                "{}",
                expression,
            );
            println!("\n{}\n=\n\n{}\n", command, output);
        }
    }
}

fn interpreter() -> ! {
    // Welcome message
    println!("{}\nVersion {}", "The Elemental Interpreter".truecolor(255, 140, 0).bold(), VERSION);

    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Store a list of variables in the program
    let mut variables = HashMap::new();

    loop {
        // Prompt the user
        print!(">>> ");
        match stdout.flush() {
            Ok(_) => (),
            Err(_) => throw(CouldNotFlushOutput),
        };

        match stdin.read_line(&mut input) {
            Ok(_) => (),
            Err(_) => throw(CouldNotReadStdin),
        };

        let (expression, is_silent) = interpret(&mut variables, input.to_owned());

        // Only if it is not "silent", display output
        if !is_silent {
            let output = format!(
                "{}",
                expression,
            );
            println!("\n{}\n", output);
        }

        input.clear();
    }
}