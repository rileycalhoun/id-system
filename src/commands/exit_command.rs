
use crate::console::LogLevel;
use crate::files::DataFiles;
use crate::log;

pub fn exit_command(_: &mut DataFiles) {
    log!(LogLevel::INFO, "Thank you for using the ID system!");
    std::process::exit(0);
}