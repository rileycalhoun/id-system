
use crate::console::LogLevel;
use crate::files::StructureFile;
use crate::log;

pub fn exit_command(_: &mut StructureFile) {
    log!(LogLevel::INFO, "Thank you for using the ID system!");
    std::process::exit(0);
}