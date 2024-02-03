use crate::{
    log, 
    models::Employee, 
    state::ProgramState
};

fn save_insertions(new_employees: &mut Vec<Employee>) -> (i16, i16) {
    let mut indexes_inserted: Vec<usize> = Vec::new();
    let mut errors: i16 = 0;

    for (index, employee) in new_employees.iter().enumerate() {
        let inserted = Employee::insert(employee);
        if !inserted {
            log!(
                crate::console::LogLevel::ERR,
                "Problem inserting employee with ID {}{}{}!",
                employee.department, employee.role, employee.identifier
            );

            errors += 1;
        } else {
            let mut real_len = indexes_inserted.len();
            if real_len > 0 {
                real_len -= 1;
            } 

            indexes_inserted.insert(
                real_len,
                index
            );
        }
    }

    for index in &indexes_inserted {
        new_employees.remove(*index);
    }

    let insertions = indexes_inserted.len() as i16;
    return (insertions, errors)
}

fn save_deletions(deleted_employees: &mut Vec<String>) -> (i16, i16) {
    let mut indexes_deleted: Vec<usize> = Vec::new();
    let mut errors: i16 = 0;
    for (index, full_identifier) in deleted_employees.iter().enumerate() {
        let deleted = Employee::remove(full_identifier);
        if !deleted {
            log!(
                crate::console::LogLevel::ERR, 
                "Problem deleting employee with ID {}!", full_identifier
            );

            errors += 1;
        } else {
            let mut real_len = indexes_deleted.len();
            if real_len > 0 {
                real_len -= 1;
            }
            
            indexes_deleted.insert(
                real_len, 
                index
            );
        }
    }

    for index in &indexes_deleted {
        deleted_employees.remove(*index);
    }

    let deletions = indexes_deleted.len() as i16;
    return (deletions, errors);
}

pub fn save_command(state: &mut ProgramState) {
    let mut errors: i16 = 0;

    let new_employees = &mut state.new_employees;
    let (insertions, insertion_errors) = save_insertions(new_employees);
    errors += insertion_errors;

    let deleted_employees = &mut state.deleted_employees;
    let (deletions, deletion_errors) = save_deletions(deleted_employees);
    errors += deletion_errors;

    if errors == 0 {
        log!(
            crate::console::LogLevel::INFO,
            "Success! Saved {}, deleted {}.", insertions, deletions
        );
    } else if insertions > 0 || deletions > 0 {
        log!(
            crate::console::LogLevel::INFO,
            "Partial success! Saved {}, deleted {}. {} errors.",
            insertions, deletions, errors
        )
    } else {
        log!(
            crate::console::LogLevel::ERR,
            "Error saving to the database! Couldn't save any of the {} requests.", errors
        )
    }
}