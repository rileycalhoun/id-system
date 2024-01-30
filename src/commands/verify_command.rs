
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::establish_connection;
use crate::console::{LogLevel,read_input};
use crate::files::StructureFile;
use crate::log;
use crate::models::Employee;
use crate::schema::employees::dsl::*;

pub fn verify_command(_: &mut StructureFile) {
    use crate::schema::employees::dsl::{
        identifier as id_q,
        department as dep_q,
        role as role_q
    };

    log!(LogLevel::INPUT, "What is the ID of the employee you'd like to verify?");
    let full_identifier = read_input();
    
    let mut uid_str = full_identifier.clone();
    uid_str.replace_range(0..4, "");

    let mut department_str: String = full_identifier.clone();
    department_str.replace_range(2..6, "");

    let mut role_str: String = full_identifier.clone();
    role_str.replace_range(0..1, "");
    role_str.replace_range(4..6, "");

    let mut connection = establish_connection();
    let result = employees
        .filter(id_q.eq(&uid_str))
        .filter(dep_q.eq(&department_str))
        .filter(role_q.eq(&role_str))
        .limit(1)
        .get_result::<Employee>(&mut connection);

    if result.is_err() {
        log!(LogLevel::INFO, "That employee doesn't exist in our records!");
        return;
    }

    let employee: Employee = result.unwrap(); 
    log!(LogLevel::INFO, "Found employee {} {} from the provided ID!", employee.first_name, employee.last_name);
}