
use crate::log;
use crate::console::{LogLevel,read_input};
use crate::files::DataFiles;

pub fn delete_command(files: &mut DataFiles) {
    log!(LogLevel::INPUT, "What is the ID of the user you'd like to remove?");
    let id = read_input();
    let optional = files.employees.remove(&id);
    match optional {
        Some(employee) => log!(LogLevel::INFO, 
            "Removed {} {} from the employee list!", employee.first_name, employee.last_name),
        None => log!(
            LogLevel::ERR, "User with ID {} does not exist!", id)
    }
}