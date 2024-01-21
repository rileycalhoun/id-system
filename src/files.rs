extern crate serde_json;

use serde::{Serialize,Deserialize};
use serde_json::{Value,Deserializer};
use std::fs;
use std::path::Path;
use std::io::BufWriter;

#[derive(Deserialize,Serialize,Clone)]
pub struct FileData {
    pub id: i32,
    pub title: String
}

#[derive(Clone)]
pub struct DataFiles {
    pub departments: Vec<FileData>,
    pub roles: Vec<FileData>
}

pub fn has_needed_files() -> bool {
    let has_data_folder = Path::new("data").exists();    
    let has_employee_file = Path::new("data/employees.json").exists();
    if has_data_folder && !has_employee_file {
        match fs::File::create("data/employees.json") {

            Ok(file) => {
                let writer = BufWriter::new(file);
                let default_value: Value = serde_json::from_str("{}").unwrap();
                serde_json::to_writer_pretty(writer, &default_value)
                    .expect("Couldn't write to employees.json!");
            }

            Err (_) => {
                return false;
            }

        }
    }

    let has_departments_file = Path::new("data/departments.json").exists();
    let has_roles_file = Path::new("data/roles.json").exists();
    return has_data_folder && has_departments_file && has_roles_file;
}

pub fn parse_file(path: &str) -> Vec<FileData> {
    let contents = fs::read_to_string(path)
        .expect(format!("Unable to call read_to_string {}!", path).as_str());

    let mut deserializer = Deserializer::from_str(&contents);
    let data = Vec::deserialize(&mut deserializer)
        .expect(format!("Could not parse data from {}", path).as_str());
    
    return data;
}

pub fn parse_data_files() -> DataFiles {
    let departments = parse_file("data/departments.json");
    let roles = parse_file("data/roles.json");

    let files = DataFiles {
        departments,
        roles
    };

    return files;
}