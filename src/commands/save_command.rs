
use crate::console::{log,LogLevel};
use crate::files::DataFiles;

pub fn save_command(files: &mut DataFiles) {
    files.employees.write()
        .expect("Could not save employees.json!");
    log(
        LogLevel::INFO,
        format!("Successfully saved employees.json!")
    );
}