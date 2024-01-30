
use crate::console::LogLevel;
use crate::state::ProgramState;
use crate::log;

pub fn help_command(_: &mut ProgramState) {
    let mut index: usize = 0;
    let commands = super::COMMANDS.clone();

    'a: loop {
        let optional = commands.get(index);
        if optional.is_none() {
            log!(
                LogLevel::ERR,
                "Error running help; looped too many times."
            );
            return;
        }

        let command = optional.unwrap();

        log!(LogLevel::INFO, "{:?}. {}", index + 1, command.name);

        index += 1;
        if index ==  commands.len() {
            break 'a;
        }
    }
}