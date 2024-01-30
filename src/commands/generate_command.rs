use std::rc::Rc;
use diesel::dsl::exists;
use diesel::query_dsl::methods::FilterDsl;
use diesel::{select, ExpressionMethods, RunQueryDsl, SelectableHelper};
use rand::Rng;

use crate::commands::pad_left;
use crate::console::{LogLevel,read_input};
use crate::files::{StructureFile, StructuredData};
use crate::schema::employees;
use crate::{establish_connection, log};
use crate::models::Employee;

fn get_id_from_title(title: String, data: Rc<[StructuredData]>) -> Option<i32> {
    for entry in data.iter() {
        if entry.title.trim()
            .to_ascii_lowercase() == title.trim().to_ascii_lowercase() {
            return Some(entry.id)
        }
    }

    return None
}

fn get_padded_string_from_data(data: Rc<[StructuredData]>, phrase: &str) -> String {
    let role: i32;
    loop {
        for data in data.iter() {
            log!(LogLevel::INPUT, "{}. {}", data.id, data.title);
        }

        log!(LogLevel::INPUT, "Please pick a {}:", phrase);
        let r = get_id_from_title(read_input(), data.clone());
        if r.is_some() {
            role = r.unwrap();
            break;
        }

        log!(
            LogLevel::INFO,
            "Unable to find a {} by that name!", phrase
        );
    }

    return pad_left(role.to_string(), 2);
}

fn gen_id() -> String {
    use crate::schema::employees::dsl::*;
    let mut rng = rand::thread_rng();
    let mut id: i16;

    let connection = &mut establish_connection();
    loop {
        id = rng.gen_range(1..1000);
        let padded_id = pad_left(id.clone().to_string(), 3);
        let employee = select(exists(
            employees.filter(identifier.eq(padded_id))
        )).get_result::<bool>(connection);
        
        if employee.is_err() || employee.unwrap() == false {
            break;
        }
    }

    return pad_left(id.to_string(), 3);
}

pub fn generate_command(file: &mut StructureFile) {
    log!(LogLevel::INPUT, "What is the employee's first name?");
    let first_name = read_input();

    log!(LogLevel::INPUT, "What is the employee's last name?");
    let last_name = read_input();

    let department = get_padded_string_from_data(file.departments.clone(), "department");
    let role = get_padded_string_from_data(file.roles.clone(), "role");
    
    // TODO: re-implement
    // let has_employee = &files.employees.get_employee_by_full_name(&first_name, &last_name)
    //     .is_some();
    // if has_employee == &true {
    //     log!(
    //         LogLevel::INPUT,
    //         "Records indicate that an employee by the name of {} {} is already assigned an employee ID, create anyways? (Y/N)",
    //         &first_name, &last_name
    //     );

    //     let continue_str = read_input();
    //     match continue_str.to_ascii_lowercase().as_str() {
    //         "y" | "yes" => {
    //             log!(
    //                 LogLevel::INFO,
    //                 "Continuing!"
    //             )
    //         },
    //         _ => {
    //             log!(
    //                 LogLevel::INFO,
    //                 "Returning..."
    //             );
    //             return
    //         }
    //     }
    // }

    let id = gen_id();
    let employee = Employee {
        identifier: id,
        department,
        role,
        first_name,
        last_name
    };

    log!(LogLevel::INFO, "Generated new ID for {} {}: {}{}{}", 
        &employee.first_name, 
        &employee.last_name, 
        &employee.department, 
        &employee.role, 
        &employee.identifier);

    let connection = &mut establish_connection();
    diesel::insert_into(employees::table)
        .values(&employee)
        .returning(Employee::as_returning())
        .get_result(connection)
        .expect("Error saving new employee!");
    log!(LogLevel::INFO, "Saved new employee in the database!");
}