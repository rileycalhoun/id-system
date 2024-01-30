use crate::console::{LogLevel,read_input};
use crate::database::{contains_by_full_identifier, get_employee};
use crate::state::ProgramState;
use crate::log;

pub fn verify_command(_: &mut ProgramState) {
    log!(LogLevel::INPUT, "What is the ID of the employee you'd like to verify?");
    let full_identifier = read_input();
    let contains = contains_by_full_identifier(&full_identifier);

    if contains {
        log!(LogLevel::INFO, "That employee doesn't exist in our records!");
        return;
    }

    let employee = get_employee(&full_identifier).unwrap();
    log!(LogLevel::INFO, "Found employee {} {} from the provided ID!", employee.first_name, employee.last_name);
}