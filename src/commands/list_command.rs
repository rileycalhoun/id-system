
use crate::console::LogLevel;
use crate::files::DataFiles;
use crate::log;

pub fn list_command(files: &mut DataFiles) {
    let employee_list = files.employees.clone().get_employees();
    let mut index = 0;
    for employee in employee_list {
        index += 1;
        let id = employee.department.clone()+ &employee.role + &employee.id; 
        log!(
            LogLevel::INFO,
            "{}. {} {}, ID: {}", index, employee.first_name, employee.last_name, id
        );
    }
}