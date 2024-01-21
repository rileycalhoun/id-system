
use crate::console::{log,LogLevel};
use crate::employee::EmployeeFile;
use crate::files::DataFiles;

pub fn exit_command(_: &DataFiles, _: &mut EmployeeFile) {
    log(LogLevel::INFO, format!("Thank you for using the ID system!"));
    std::process::exit(0);
}