
use crate::console::{log,LogLevel};
use crate::employee::EmployeeFile;
use crate::files::DataFiles;

pub fn list_command(_: &DataFiles, employees: &mut EmployeeFile) {
    let employee_list = employees.clone().json();
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