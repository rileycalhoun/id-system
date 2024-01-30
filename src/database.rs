use std::env;
use diesel::associations::HasTable;
use diesel::result::Error;
use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;

use crate::schema::employees::dsl::*;
use crate::models::Employee;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("Unable to find DATABASE URL in .env file");

    return PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
}

pub fn get_employees() -> Vec<Employee> {
    let connection = &mut establish_connection();
    return employees
        .load::<Employee>(connection)
        .expect("couldn't find employee list");
}

pub fn insert_employee(employee: &Employee) -> bool {
    let connection = &mut establish_connection();
    let result = diesel::insert_into(employees::table())
        .values(employee)
        .returning(Employee::as_returning())
        .get_result(connection);
    if result.is_ok() {
        return true;
    } else {
        return false;
    }
}

pub fn get_employee(full_identifier: &String) -> Option<Employee> {
    let connection = &mut establish_connection();
    let mut uid = full_identifier.clone();
    uid.replace_range(0..4, "");
    let result = employees
        .filter(identifier.eq(uid))
        .limit(1)
        .get_result::<Employee>(connection);
    
    if result.is_ok() {
        return Some(result.unwrap())
    } else {
        return None
    }
}

pub fn delete_employee(full_identifier: &String) -> bool { 
    let connection = &mut establish_connection(); 
    let mut uid = full_identifier.clone();
    uid.replace_range(0..4, "");

    let deleted: Result<usize, Error> = diesel::delete(
        employees::table().find(uid)
    ).execute(connection);
    return deleted.is_ok();
}

pub fn contains_by_unique_identifier(uid: &String) -> bool {
    let connection = &mut establish_connection();
    let result = employees
        .filter(identifier.eq(uid))
        .limit(1)
        .get_result::<Employee>(connection);
    if result.is_err() {
        return false;
    } else {
        return true;
    }
}

pub fn contains_by_full_identifier(full_identifier: &String) -> bool {
    let mut uid = full_identifier.clone();
    if uid.len() != 7 {
        return false
    }

    uid.replace_range(0..4, "");
    println!("{}", uid);
    return contains_by_unique_identifier(&uid); 
}

pub fn contains_by_full_name(first_name_str: &String, last_name_str: &String) -> bool {
    let connection = &mut establish_connection();
    let result = employees
        .filter(first_name.eq(first_name_str))
        .filter(last_name.eq(last_name_str))
        .limit(1)
        .get_result::<Employee>(connection);
    return result.is_ok();
}