
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::result::Error;

use crate::{establish_connection, log};
use crate::console::{LogLevel,read_input};
use crate::files::StructureFile;

pub fn delete_command(_: &mut StructureFile) {
    use crate::schema::employees::dsl::*;

    log!(LogLevel::INPUT, "What is the ID of the user you'd like to remove?");
    let original_id = read_input();
    let mut uid = original_id.clone();
    uid.replace_range(0..4, "");
    
    let mut connection = establish_connection();
    let deleted: Result<usize, Error> = diesel::delete(
        employees::table().find(uid)
    )
    .execute(&mut connection);

    match deleted {
        Ok(_) => {
            log!(
                LogLevel::INFO,
                "Successfully deleted employee with ID {}!", original_id
            )
        },
        Err(_) => {
            log!(
                LogLevel::ERR,
                "Unable to find employee with ID {}!", original_id
            )
        }
    }
}