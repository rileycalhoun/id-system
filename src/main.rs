mod commands;
mod console;
mod employee;
mod files; 

extern crate serde_json;

use std::path::Path;

use commands::{cook_raw_command,handle_command};
use console::{read_input,log,LogLevel};
use employee::EmployeeFile;
use files::{has_needed_files, parse_file, DataFiles};

fn main() {
    if !has_needed_files() {
        log(
            LogLevel::ERR,
            format!("You need to have the 'data' directory provided with the system in the same folder when running this application.")
        );
        return;
    }

    let employees = EmployeeFile::new(Path::new("data/employees.json"));
    let departments = parse_file("data/departments.json");
    let roles = parse_file("data/roles.json");

    let mut files = DataFiles {
        departments,
        roles,
        employees
    };

    log(LogLevel::INFO, format!("Welcome to the Employee ID System!"));

    loop {
        let raw_command = read_input();
        let command = cook_raw_command(raw_command);
        handle_command(command, &mut files);
    }
}