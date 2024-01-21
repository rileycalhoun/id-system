pub mod delete_command;
pub mod exit_command;
pub mod generate_command;
pub mod help_command;
pub mod list_command;
pub mod save_command;
pub mod verify_command;

use crate::console::{log,LogLevel};
use crate::employee::EmployeeFile;
use crate::files::DataFiles;

use self::delete_command::delete_command;
use self::exit_command::exit_command;
use self::generate_command::generate_command;
use self::help_command::help_command;
use self::list_command::list_command;
use self::save_command::save_command;
use self::verify_command::verify_command;

pub struct Command {
    name: String,
    aliases: Option<&'static [&'static str]>,
    exec: fn(args: Vec<String>, files: &DataFiles, employees: &mut EmployeeFile)
}

impl Command {

    fn new(name: &str, aliases: Option<&'static [&str]>, exec: fn(args: Vec<String>, files: &DataFiles, employees: &mut EmployeeFile)) -> Self {
        Command {
            name: String::from(name.to_ascii_lowercase()),
            aliases,
            exec
        }
    }

}

pub fn cook_raw_command(raw_command: String) -> (String, Vec<String>) {
    let command_array: Vec<&str> = raw_command
        .split(" ")
        .collect::<Vec<&str>>();

    let command = command_array.get(0)
        .expect("Did not provide a command!")
        .trim()
        .to_ascii_lowercase();
        
    let mut args = command_array
        .clone()
        .into_iter()
        .map(|arg| arg.trim().to_ascii_lowercase())
        .collect::<Vec<String>>();

    args.remove(0);
    return (command, args)
}

pub fn handle_command(command: String, args: Vec<String>, files: &DataFiles, employees: &mut EmployeeFile) {

    let optional = get_command(command.to_ascii_lowercase());
    match optional {
        Some(command) => {
            (command.exec)(args, files, employees);
        }

        None => {
            log(LogLevel::INFO, format!("Unknown command! Try 'help' for a list of commands"));
        }
    }

}

pub fn get_command(command_name: String) -> Option<Command> {
    for command in get_commands() {
        if command.name == command_name {
            return Some(command)
        }

        if command.aliases.is_some() {
            for alias in command.aliases.unwrap() {
                if alias.to_ascii_lowercase() == command_name {
                    return Some(command)
                }
            }
        }
    }

    return None
}

pub fn get_commands() -> [Command; 7] {
    return [
        Command::new("generate", None, generate_command),
        Command::new("verify", None, verify_command),
        Command::new("delete", None, delete_command),
        Command::new("save", None, save_command),
        Command::new("help", None, help_command),
        Command::new("exit", Some(&["close", "stop"]), exit_command),
        Command::new("list", None, list_command)
    ];
}