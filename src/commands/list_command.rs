use std::iter::Filter;

use crate::models::Employee;
use crate::console::LogLevel;
use crate::state::ProgramState;
use crate::log;

pub fn list_command(state: &mut ProgramState) {
    let mut employee_list = Employee::get_all();
    
    // O(n) or O(n^2)?
    // Try to come up with a different algorithm for deleting these entries
    for identifier in &state.deleted_employees {
        employee_list = employee_list.iter()
            .cloned()
            .filter(|e| {
                let mut uid = identifier.clone();
                uid.replace_range(0..4, "");

                return e.identifier != uid;
            })
            .collect();
    }

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

    if &state.new_employees.len() > &0 {
        log!(LogLevel::INFO, "Unsaved Entries:");
        for employee in &state.new_employees {
            log!(
                LogLevel::INFO,
                "{}. {} {}, ID {}{}{}",
                index, employee.first_name, employee.last_name,
                employee.department, employee.role, employee.identifier
            );
        }
    }
}