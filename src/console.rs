use std::io::{stdin, self, Write};

pub enum LogLevel {
    INFO, ERR, INPUT
}

#[macro_export]
macro_rules! log {
    (
        // name of the value from the LogLevel enum
        $name:path,
        $($arg:tt)*

    ) => {
        $crate::console::_log(
            $crate::console::LogLevel::from($name),
            format!($($arg)*)
        )
    }
}

#[doc(hidden)]
pub fn _log(level: LogLevel, message: String) {
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
