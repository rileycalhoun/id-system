
use crate::console::LogLevel;
use crate::files::DataFiles;
use crate::log;

pub fn save_command(files: &mut DataFiles) {
    files.employees.write()
        .expect("Could not save employees.json!");
    log!(
        LogLevel::INFO,
        "Successfully saved employees.json!"
    );
}