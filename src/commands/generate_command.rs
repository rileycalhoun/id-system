use rand::Rng;

use crate::console::{log,LogLevel,read_input};
use crate::employee::Employee;
use crate::files::{DataFiles, FileData};

fn get_id(title: String, file: Vec<FileData>) -> Option<i32> {
    for entry in file {
        if entry.title.trim()
            .to_ascii_lowercase() == title.trim().to_ascii_lowercase() {
            return Some(entry.id)
        }
    }

    return None
}

fn ensure_length(id: String, len: usize) -> String {
    let mut new_id = id.clone();
    while new_id.len() < len {
        new_id.insert_str(0, "0");
    }

    return new_id;
}

pub fn generate_command(files: &mut DataFiles) {
    let department: i32;
    'a: loop {
        for data in files.departments.clone() {
            log(LogLevel::INPUT,
                format!("{}. {}", data.id, data.title));
        }
        
        log(LogLevel::INPUT, format!("Please pick a department: "));
        let dep = get_id(read_input(), files.departments.clone());
        if dep.is_some() {
            department = dep.unwrap();
            break 'a;
        }

        log(
            LogLevel::INFO,
            format!("Unable to find a department by that name!")
        );
    }

    let role: i32;
    'a: loop {
        for data in files.roles.clone() {
            log(LogLevel::INPUT, format!("{}. {}", data.id, data.title));
        }

        log(LogLevel::INPUT, format!("Please pick a role:"));
        let r = get_id(read_input(), files.roles.clone());
        if r.is_some() {
            role = r.unwrap();
            break 'a;
        }

        log(
            LogLevel::INFO,
            format!("Unable to find a role by that name!")
        );
    }

    log(LogLevel::INPUT, format!("What is the employee's first name?"));
    let first_name = read_input();

    log(LogLevel::INPUT, format!("What is the employee's last name?"));
    let last_name = read_input();

    let has_employee = &files.employees.get_employee_by_full_name(&first_name, &last_name)
        .is_some();
    if has_employee == &true {
        log(
            LogLevel::INPUT,
            format!("Records indicate that an employee by the name of {} {} is already assigned an employee ID, create anyways? (Y/N)",
                        &first_name, &last_name)
        );

        let continue_str = read_input();
        match continue_str.to_ascii_lowercase().as_str() {
            "y" | "yes" => {
                log(
                    LogLevel::INFO,
                    format!("Continuing!")
                )
            },
            _ => {
                log(
                    LogLevel::INFO,
                    format!("Returning...")
                );
                return
            }
        }
    }

    // TODO: Check whether an employee with the first and last name exist in the database already

    let mut rng = rand::thread_rng();
    let mut id;

    'a: loop {
        id = rng.gen_range(0..1000).to_string();
        if !&files.employees.contains(&id) {
            break 'a;
        }
    }

    let employee = Employee {
        first_name,
        last_name,
        department: ensure_length(department.to_string(), 2),
        role: ensure_length(role.to_string(), 2),
        id: ensure_length(id, 2)
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
    files.employees.insert(employee);
}