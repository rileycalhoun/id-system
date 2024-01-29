
use crate::console::LogLevel;
use crate::files::DataFiles;
use crate::log;

pub fn help_command(_: &mut DataFiles) {
    let mut index: usize = 0;
    let commands = &super::get_commands();
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
        if index ==  super::get_commands().len() {
            break 'a;
        }
    }
}