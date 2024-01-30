pub mod delete_command;
pub mod exit_command;
pub mod generate_command;
pub mod help_command;
pub mod list_command;
pub mod save_command;
pub mod verify_command;

use self::delete_command::delete_command;
use self::exit_command::exit_command;
use self::generate_command::generate_command;
use self::help_command::help_command;
use self::list_command::list_command;
use self::save_command::save_command;
use self::verify_command::verify_command;

use std::sync::Arc;
use lazy_static::lazy_static;

use crate::console::LogLevel;
use crate::state::{ProgramState,StructureFile};
use crate::log;

pub struct Command {
    name: Arc<str>,
    aliases: Arc<[Arc<str>]>,
    exec: fn(data: &mut ProgramState)
}

impl Command {

    fn new(
        name: &str,
        aliases: &[&str],
        exec: fn(data: &mut ProgramState)
    ) -> Self {
        let aliases = aliases.iter()
            .map(|s| Arc::from(s.to_owned()))
            .collect::<Arc<[Arc<str>]>>();

        Self {
            name: Arc::from(name),
            aliases,
            exec
        }
    }

}

lazy_static! {
    pub static ref COMMANDS: Arc<[Command]> = {
        return Arc::from([
            Command::new("generate", CommandHandler::NO_ALIASES, generate_command),
            Command::new("verify", CommandHandler::NO_ALIASES, verify_command),
            Command::new("delete", CommandHandler::NO_ALIASES, delete_command),
            Command::new("help", CommandHandler::NO_ALIASES, help_command),
            Command::new("exit", &["close", "stop"], exit_command),
            Command::new("list", CommandHandler::NO_ALIASES, list_command),
            Command::new("save", CommandHandler::NO_ALIASES, save_command)
        ])
    };
}

pub struct CommandHandler {
    state: ProgramState
}

impl CommandHandler {
    
    const NO_ALIASES: &'static [&'static str] = &[]; 

    pub fn new(structure_file: StructureFile) -> Self {
        return Self {
            state: ProgramState {
                structure_file,
                new_employees: Vec::new(),
                deleted_employees: Vec::new()
            }
        }
    }

    pub fn get_command(&self, command_name: String) -> Option<&Command> {
        for command in COMMANDS.into_iter() {
            if command.name.to_string() == command_name {
                return Some(command)
            }
    
            for alias in command.aliases.iter() {
                if alias.to_string() == command_name {
                    return Some(command)
                }
            }
        }
    
        return None
    }

    pub fn handle_command(&mut self, command: String) {

        let optional = self.get_command(command.to_ascii_lowercase());
        match optional {
            Some(command) => {
                (command.exec)(&mut self.state);
            }
    
            None => {
                log!(LogLevel::INFO, "Unknown command! Try 'help' for a list of commands");
            }
        }
    
    }

}

pub fn cook_raw_command(raw_command: String) -> String {
    let command_array: Vec<&str> = raw_command
        .split(" ")
        .collect::<Vec<&str>>();

    let command = command_array.get(0)
        .expect("Did not provide a command!")
        .trim()
        .to_ascii_lowercase();

    return command
}

pub(crate) fn pad_left(unpadded: String, len: usize) -> String {
    let mut padded = unpadded.clone();

    while padded.len() < len {
        padded.insert(0, '0');
    }

    return padded;
}