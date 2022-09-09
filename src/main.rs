//! Main executable for the Elemental language interpreter.

use std::{
    io::{
        self,
        Write,
    },
    collections::HashMap,
};

use colored::*;

use elemental::interpret;

const VERSION: &str = "0.1.0";

fn main() -> ! {
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
            Err(_) => todo!(),
        };

        match stdin.read_line(&mut input) {
            Ok(_) => (),
            Err(_) => todo!(),
        };

        println!("{}\n", interpret(&mut variables, input.to_owned()));

        input.clear();
    }
}