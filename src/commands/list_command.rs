use crate::models::Employee;
use crate::console::LogLevel;
use crate::state::ProgramState;
use crate::log;

pub fn list_command(_: &mut ProgramState) {
    let employee_list = Employee::get_all();

    let mut index = 1;
    for employee in employee_list {
        log!(
            LogLevel::INFO,
            "{}. {} {}, ID: {}{}{}", 
            index, employee.first_name, employee.last_name, 
            employee.department, employee.role, employee.identifier
        );

        index += 1;
    }
}