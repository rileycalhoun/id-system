use crate::{files::StructureFile, console::LogLevel};
use crate::log;

pub fn reload_command(_: &mut StructureFile) {

    // files.employees.reload()
    //     .expect("Error; Unable to reload employees.json!");
    // log!(
    //     LogLevel::INFO, 
    //     "Successfuly reloaded the employees.json file!"
    // );

    log!(
        LogLevel::ERR,
        "Reload functionality hasn't been reimplemented yet."
    )

}