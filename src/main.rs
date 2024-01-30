use std::path::Path;
use id_system::database::establish_connection;
use id_system::log;
use id_system::commands::{cook_raw_command,CommandHandler};
use id_system::console::{read_input,LogLevel};
use id_system::state::StructureFile;


fn main() {
    let has_data_file = Path::new("structure.json").exists();
    if !has_data_file {
        log!(
            LogLevel::ERR,
            "You need to have the structure.json file in the same directory as the application!"
        );
        return;
    }

    let _ = establish_connection(); 
    let structure_file = StructureFile::parse("structure.json");

    log!(LogLevel::INFO, "Welcome to the Employee ID System!");

    let mut handler = CommandHandler::new(structure_file);
    loop {
        let raw_command = read_input();
        let command = cook_raw_command(raw_command);
        handler.handle_command(command);
    };
}