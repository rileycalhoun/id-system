extern crate serde_json;

use serde::{Serialize,Deserialize};
use serde_json::Deserializer;
use std::fs;
use std::rc::Rc;

#[derive(Deserialize,Serialize,Clone)]
pub struct StructuredData {
    pub id: i32,
    pub title: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct StructureFile {
    pub departments: Rc<[StructuredData]>,
    pub roles: Rc<[StructuredData]>
}

impl StructureFile {

    pub fn parse(path: &str) -> Self {
        let contents = fs::read_to_string(path)
            .expect(format!("Unable to call read_to_string {}!", path).as_str());
    
        let mut deserializer = Deserializer::from_str(&contents);
        
        return StructureFile::deserialize(&mut deserializer)
            .expect(format!("Could not parse data from {}", path).as_str())
    }

}