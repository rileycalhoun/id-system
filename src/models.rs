use crate::establish_connection;
use crate::schema::employees::dsl::*;
use diesel::{associations::HasTable, prelude::*, result::Error};

#[derive(Queryable, Selectable, Insertable,Clone)]
#[diesel(table_name = crate::schema::employees)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Employee {
    pub identifier: String,
    pub department: String,
    pub role: String,
    pub first_name: String,
    pub last_name: String,
}

impl Employee {

    pub fn get_all() -> Vec<Employee> {
        let connection = &mut establish_connection();
        return employees
            .load::<Employee>(connection)
            .expect("couldn't find employee list");
    }
    
    pub fn get_by_full_id(full_identifier: &String) -> Option<Employee> {
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

    pub fn insert(employee: &Employee) -> bool {
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
    
    pub fn remove(full_identifier: &String) -> bool { 
        let connection = &mut establish_connection(); 
        let mut uid = full_identifier.clone();
        uid.replace_range(0..4, "");
    
        let deleted: Result<usize, Error> = diesel::delete(
            employees::table().find(uid)
        ).execute(connection);
        return deleted.is_ok();
    }
    
    pub fn has_by_uid(uid: &String) -> bool {
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
    
    pub fn has_by_full_id(full_identifier: &String) -> bool {
        let mut uid = full_identifier.clone();
        if uid.len() != 7 {
            return false
        }
    
        uid.replace_range(0..4, "");
        println!("{}", uid);
        return Self::has_by_uid(&uid); 
    }
    
    pub fn has_by_full_name(first_name_str: &String, last_name_str: &String) -> bool {
        let connection = &mut establish_connection();
        let result = employees
            .filter(first_name.eq(first_name_str))
            .filter(last_name.eq(last_name_str))
            .limit(1)
            .get_result::<Employee>(connection);
        return result.is_ok();
    }

}
