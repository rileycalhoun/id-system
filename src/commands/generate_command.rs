use rand::Rng;

use crate::console::{log,LogLevel,read_input};
use crate::employee::{EmployeeFile,Employee};
use crate::files::DataFiles;

pub fn generate_command(files: &DataFiles, employees: &mut EmployeeFile) {
    for data in files.departments.clone() {
        log(LogLevel::INPUT,
            format!("{}. {}", data.id, data.title));
    }

    log(LogLevel::INPUT, format!("Please pick a department: "));
    let department = read_input();

    for data in files.roles.clone() {
        log(LogLevel::INPUT, format!("{}. {}", data.id, data.title));
    }
    log(LogLevel::INPUT, format!("Please pick a role: "));
    let role = read_input();
    
    log(LogLevel::INPUT, format!("What is the employee's first name?"));
    let first_name = read_input();

    log(LogLevel::INPUT, format!("What is the employee's last name?"));
    let last_name = read_input();

    let mut rng = rand::thread_rng();
    let mut id;

    'a: loop {
        id = rng.gen_range(0..1000).to_string();
        if !employees.contains(&id) {
            break 'a;
        }
    }

    let employee = Employee {
        first_name,
        last_name,
        department,
        role,
        id
    };

    log(LogLevel::INFO, format!("Generated new ID for {} {}: {}{}{}", 
        &employee.first_name, 
        &employee.last_name, 
        &employee.department, 
        &employee.role, 
        &employee.id));

    log(
        LogLevel::INFO, 
        format!("Be sure to save the file before closing the provram with the 'save' command.")
    );
    employees.insert(employee);
}