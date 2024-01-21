
use crate::console::{log,LogLevel};
use crate::employee::EmployeeFile;
use crate::files::DataFiles;

pub fn save_command(_: Vec<String>, _: &DataFiles, employees: &mut EmployeeFile) {
    employees.write()
        .expect("Could not save employees.json!");
    log(
        LogLevel::INFO,
        format!("Successfully saved employees.json!")
    );
}