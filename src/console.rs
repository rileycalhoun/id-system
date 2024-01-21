use std::io::{stdin, self, Write};

// TODO: WARN is never used. If we need it, add it.
pub enum LogLevel {
    INFO, ERR, INPUT
}

pub fn log(level: LogLevel, message: String) {
    match level {
        LogLevel::INFO => println!("INFO | {}", message),
        LogLevel::ERR => println!("ERROR | {}", message),
        LogLevel::INPUT => println!("INPUT | {}", message)
    }
}

pub fn read_input() -> String {
    let stdin = stdin();
    let mut input: String = String::new();
    
    print!("> ");
    io::stdout().flush()
        .expect("Error; unable to flush console.");

    stdin.read_line(&mut input)
        .expect("Invalid input provided!");

    filter_input(&mut input);

    return input;
}

fn filter_input(input: &mut String) {
    let mut was_mutated = false;

    if input.ends_with("\n") {
        input.pop();
        was_mutated = true;
    }

    if input.ends_with("\r") {
        input.pop();
        was_mutated = true;
    }

    if input.ends_with("\\") {
        input.pop();
        was_mutated = true;
    }

    if was_mutated {
        filter_input(input);
    }
}