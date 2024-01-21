
use crate::console::{log,LogLevel};
use crate::files::DataFiles;

pub fn list_command(files: &mut DataFiles) {
    let employee_list = files.employees.clone().get_employees();
    let mut index = 0;
    for employee in employee_list {
        index += 1;
        let id = employee.department.clone()+ &employee.role + &employee.id; 
        log(
            LogLevel::INFO,
            format!("{}. {} {}, ID: {}", index, employee.first_name, employee.last_name, id)
        );
    }
}