
use crate::console::{log,LogLevel};
use crate::files::DataFiles;

pub fn help_command(_: &mut DataFiles) {
    let mut index: usize = 0;
    let commands = &super::get_commands();
    'a: loop {
        let optional = commands.get(index);
        if optional.is_none() {
            log(
                LogLevel::ERR,
                format!("Error running help; looped too many times.")
            );
            return;
        }

        let command = optional.unwrap();

        log(LogLevel::INFO, format!("{:?}. {}", index + 1, command.name));

        index += 1;
        if index ==  super::get_commands().len() {
            break 'a;
        }
    }
}