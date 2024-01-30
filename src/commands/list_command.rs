
use diesel::RunQueryDsl;

use crate::console::LogLevel;
use crate::files::StructureFile;
use crate::models::Employee;
use crate::{establish_connection, log};

pub fn list_command(_: &mut StructureFile) {
    use crate::schema::employees::dsl::*; 
    let mut connection = establish_connection();
    let employee_list = employees
        .load::<Employee>(&mut connection)
        .expect("couldn't find employee list");

    let mut index = 1;
    for employee in employee_list.iter() {
        log!(
            LogLevel::INFO,
            "{}. {} {}, ID: {}", index, employee.first_name, employee.last_name, employee.identifier
        );

        index += 1;
    }
}