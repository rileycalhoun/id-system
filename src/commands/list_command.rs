use crate::console::LogLevel;
use crate::database::get_employees;
use crate::state::ProgramState;
use crate::log;

pub fn list_command(_: &mut ProgramState) {
    let employee_list = get_employees();

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