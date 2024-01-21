
use crate::console::{log,LogLevel,read_input};
use crate::employee::EmployeeFile;
use crate::files::DataFiles;

pub fn delete_command(_: &DataFiles, employees: &mut EmployeeFile) {
    log(LogLevel::INPUT, format!("What is the ID of the user you'd like to remove?"));
    let id = read_input();
    let optional = employees.remove(&id);
    match optional {
        Some(employee) => log(LogLevel::INFO, 
            format!("Removed {} {} from the employee list!", employee.first_name, employee.last_name)),
        None => log(
            LogLevel::ERR, format!("Could not remove id: {} does not exist!", id))
    }
}