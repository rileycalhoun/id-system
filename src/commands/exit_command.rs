
use crate::console::{log,LogLevel};
use crate::files::DataFiles;

pub fn exit_command(_: &mut DataFiles) {
    log(LogLevel::INFO, format!("Thank you for using the ID system!"));
    std::process::exit(0);
}