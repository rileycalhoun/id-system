
use crate::console::{log,LogLevel,read_input};
use crate::files::DataFiles;

pub fn verify_command(files: &mut DataFiles) {
    log(LogLevel::INPUT, format!("What is the ID of the employee you'd like to verify?"));
    let id = read_input();
    if !&files.employees.contains(&id) {
        log(LogLevel::INFO, format!("That employee doesn't exist in our records!"));
        return;
    }

    let (employee, _)= &files.employees.get_employee(&id)
        .expect("Something went wrong; Checked for Employee ID but still found nothing.");
    log(LogLevel::INFO, format!("Found employee {} {} from the provided ID!", employee.first_name, employee.last_name));
}