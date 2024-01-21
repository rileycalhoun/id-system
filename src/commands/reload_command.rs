use crate::{files::DataFiles, console::{log,LogLevel}};

pub fn reload_command(files: &mut DataFiles) {

    files.employees.reload()
        .expect("Error; Unable to reload employees.json!");
    log(
        LogLevel::INFO, 
        format!("Successfuly reloaded the employees.json file!")
    );

}