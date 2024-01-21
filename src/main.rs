pub mod commands;
pub mod console;
pub mod employee;
pub mod files;

extern crate serde_json;

use commands::{cook_raw_command,handle_command};
use console::{read_input,log,LogLevel};
use employee::EmployeeFile;
use files::{has_needed_files, parse_data_files};

fn main() {
    if !has_needed_files() {
        log(
            LogLevel::ERR,
            format!("You need to have the 'data' directory provided with the system in the same folder when running this application.")
        );
        return;
    }

    let files = parse_data_files();
    let mut employees = EmployeeFile::new();

    log(LogLevel::INFO, format!("Welcome to the Employee ID System"));

    loop {
        let raw_command = read_input();
        let (command, args) = cook_raw_command(raw_command);
        handle_command(command, args, &files, &mut employees);
    }
}