extern crate serde_json;
use serde::{Serialize,Deserialize};
use std::{fs::File, io::{BufWriter, Write}, path::{Path, PathBuf}};

#[derive(Serialize,Deserialize,Clone)]
pub struct Employee {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub department: String
}

#[derive(Clone)]
pub struct EmployeeFile {
    path: PathBuf,
    data: Vec<Employee>
}

impl EmployeeFile {

    pub fn new(path: &Path) -> Self {
        File::open(&path)
            .expect("Could not open employees.json!");

        let contents = std::fs::read_to_string(&path)
            .expect("Could not open employees.json!");

        if contents == "{}" {
            return EmployeeFile {
                path: path.to_path_buf(),
                data: Vec::new()
            }
        }

        let data = serde_json::from_str(&contents)
            .expect("Could not read employees.json!");

        EmployeeFile {
            path: path.to_path_buf(),
            data
        }
    }

    pub fn write(&self) -> std::io::Result<()> {
        let file = File::create(&self.path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &self.data)?;
        writer.flush()?;
        Ok(())
    }

    pub fn reload(&mut self) -> std::io::Result<()> {
        let contents = std::fs::read_to_string(&self.path)?;
        self.data = serde_json::from_str(&contents)?;
        Ok(())
    }

    pub fn get_employees(self) -> Vec<Employee> {
        return self.data
    }

    pub fn get_employee(&self, id: &String) -> Option<(Employee, usize)> {
        let mut index: usize = 0;
        let json = self.data.clone();
        
        if json.len() == 0 {
            return None
        }

        'a: loop {
            let employee =  json.get(index)
                .expect("Something went wrong; get_employee loop went too far.")
                .clone();
            
            let employee_id = employee.clone().department + &employee.role + &employee.id;
            if employee_id == id.to_string() {
                return Some((employee, index));
            }

            index += 1;
            if index == self.data.len() {
                break 'a;
            }
        }

        return None
    }

    pub fn contains(&self, id: &String) -> bool {
        return self.get_employee(&id).is_some();
    }

    pub fn insert(&mut self, employee: Employee) -> Option<usize> {
        if !self.contains(&employee.id) {
            self.data.push(employee);
            return Some(self.data.len() - 1)
        } else {
            return None
        }
    }

    pub fn remove(&mut self, id: &String) -> Option<Employee> {
        let optional = self.get_employee(id);
        match optional {
            Some((employee, index)) => {
                let _ = self.data.remove(index);
                return Some(employee)
            },
            None => return None
        }
    }
}