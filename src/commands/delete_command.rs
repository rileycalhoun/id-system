use crate::database::contains_by_full_identifier;
use crate::log;
use crate::console::{LogLevel,read_input};
use crate::state::ProgramState;

pub fn delete_command(state: &mut ProgramState) {
    log!(LogLevel::INPUT, "What is the ID of the user you'd like to remove?");
    let full_identifier = read_input();
    let contains = contains_by_full_identifier(&full_identifier);
    if contains {
        state.deleted_employees.push(full_identifier.clone());
        log!(
            LogLevel::INFO,
            "Successfully deleted employee with ID {}!", full_identifier
        );
    } else { 
        log!(
            LogLevel::ERR,
            "Unable to find employee with ID {}!", full_identifier
        );
    }
}