use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::employees)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Employee {
    pub identifier: String,
    pub department: String,
    pub role: String,
    pub first_name: String,
    pub last_name: String,
}