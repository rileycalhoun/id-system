use crate::{files::DataFiles, console::LogLevel};
use crate::log;

pub fn reload_command(files: &mut DataFiles) {

    files.employees.reload()
        .expect("Error; Unable to reload employees.json!");
    log!(
        LogLevel::INFO, 
        "Successfuly reloaded the employees.json file!"
    );

}