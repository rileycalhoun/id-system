use std::io::stdin;

pub enum LogLevel {
    INFO, WARN, ERR
}

pub fn log(level: LogLevel, message: &str) {
    match level {
        LogLevel::INFO => println!("INFO | {}", message),
        LogLevel::WARN => println!("WARN | {}", message),
        LogLevel::ERR => println!("ERROR | {}", message)
    }
}

pub fn read_input() -> String {
    let stdin = stdin();
    let mut input: String = String::new();
    stdin.read_line(&mut input)
        .expect("Invalid input provided!");

    if input.ends_with("\n") {
        input.pop();
    }

    return input;
}