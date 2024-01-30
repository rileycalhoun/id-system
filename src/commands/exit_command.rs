
use crate::console::LogLevel;
use crate::state::ProgramState;
use crate::log;

pub fn exit_command(_: &mut ProgramState) {
    log!(LogLevel::INFO, "Thank you for using the ID system!");
    std::process::exit(0);
}