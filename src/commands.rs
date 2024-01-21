
use rand::Rng;

use crate::{console::{log,LogLevel,read_input}, employee::{EmployeeFile, Employee}, files::DataFiles};

pub struct Command {
    name: String,
    aliases: Option<&'static [&'static str]>,
    exec: fn(args: Vec<String>, files: &DataFiles, employees: &mut EmployeeFile)
}

impl Command {

    fn new(name: &str, aliases: Option<&'static [&str]>, exec: fn(args: Vec<String>, files: &DataFiles, employees: &mut EmployeeFile)) -> Self {
        Command {
            name: String::from(name.to_ascii_lowercase()),
            aliases,
            exec
        }
    }

}

pub fn cook_raw_command(raw_command: String) -> (String, Vec<String>) {
    let command_array: Vec<&str> = raw_command
        .split(" ")
        .collect::<Vec<&str>>();

    let command = command_array.get(0)
        .expect("Did not provide a command!")
        .trim()
        .to_ascii_lowercase();
        
    let mut args = command_array
        .clone()
        .into_iter()
        .map(|arg| arg.trim().to_ascii_lowercase())
        .collect::<Vec<String>>();

    args.remove(0);
    return (command, args)
}

pub fn handle_command(command: String, args: Vec<String>, files: &DataFiles, employees: &mut EmployeeFile) {

    let optional = get_command(command.to_ascii_lowercase());
    match optional {
        Some(command) => {
            (command.exec)(args, files, employees);
        }

        None => {
            log(LogLevel::INFO, "Unknown command! Try 'help' for a list of commands.".to_string());
        }
    }

}

pub fn get_command(command_name: String) -> Option<Command> {
    for command in get_commands() {
        if command.name == command_name {
            return Some(command)
        }

        if command.aliases.is_some() {
            for alias in command.aliases.unwrap() {
                if alias.to_ascii_lowercase() == command_name {
                    return Some(command)
                }
            }
        }
    }

    return None
}

// TODO: Figure out how to separate commands
pub fn get_commands() -> [Command; 7] {
    return [
        Command::new(
            "generate",
            None,
            |_, files, employees| {
                for data in files.departments.data.clone() {
                    log(LogLevel::INPUT,
                        format!("{}. {}", data.id, data.title));
                }
                log(LogLevel::INPUT, format!("Please pick a department: "));
                let department = read_input();

                for data in files.roles.data.clone() {
                    log(LogLevel::INPUT, format!("{}. {}", data.id, data.title));
                }
                log(LogLevel::INPUT, format!("Please pick a role: "));
                let role = read_input();
                
                log(LogLevel::INPUT, format!("What is the employee's first name?"));
                let first_name = read_input();

                log(LogLevel::INPUT, format!("What is the employee's last name?"));
                let last_name = read_input();

                let mut rng = rand::thread_rng();
                let mut id;

                'a: loop {
                    id = rng.gen_range(0..1000).to_string();
                    if !employees.contains(&id) {
                        break 'a;
                    }
                }

                let employee = Employee {
                    first_name,
                    last_name,
                    department,
                    role,
                    id
                };

                log(LogLevel::INFO, format!("Generated new ID for {} {}: {}{}{}", 
                    &employee.first_name, 
                    &employee.last_name, 
                    &employee.department, 
                    &employee.role, 
                    &employee.id));

                log(
                    LogLevel::INFO, 
                    format!("Be sure to save the file before closing the provram with the 'save' command.")
                );
                employees.insert(employee);
        }),
        Command::new(
            "verify",
            None, 
            |_, _, employees| {
                log(LogLevel::INPUT, "What is the ID of the employee you'd like to verify?".to_string());
                let id = read_input();
                if !employees.contains(&id) {
                    log(LogLevel::INFO, "That employee doesn't exist in our records!".to_string());
                    return;
                }

                let (employee, _)= employees.get_employee(&id)
                    .expect("Something went wrong; Checked for Employee ID but still found nothing.");
                log(LogLevel::INFO, format!("Found employee {} {} from the provided ID!", employee.first_name, employee.last_name));
        }),
        Command::new(
            "delete",
            None,
            |_, _, employees| {
                log(LogLevel::INPUT, format!("What is the ID of the user you'd like to remove?"));
                let id = read_input();
                let optional = employees.remove(&id);
                match optional {
                    Some(employee) => log(LogLevel::INFO, 
                        format!("Removed {} {} from the employee list!", employee.first_name, employee.last_name)),
                    None => log(
                        LogLevel::ERR, format!("Could not remove id: {} does not exist!", id))
                }
            }),
        Command::new(
            "save",
            None,
            |_, _, employees| {
                employees.write()
                    .expect("Could not save employees.json!");
                log(
                    LogLevel::INFO,
                    "Successfully saved employees.json!".to_string()
                );
        }),
        Command::new(
            "help",
            None,
            |_, _, _| {
                let mut index: usize = 0;
                let commands = &get_commands();
                'a: loop {
                    let optional = commands.get(index);
                    if optional.is_none() {
                        log(
                            LogLevel::ERR,
                            "Error running help; looped too many times.".to_string()
                        );
                        return;
                    }

                    let command = optional.unwrap();

                    log(LogLevel::INFO, format!("{:?}. {}", index + 1, command.name));

                    index += 1;
                    if index ==  get_commands().len() {
                        break 'a;
                    }
                }
        }),
        Command::new("exit",
        Some(&["close", "stop"]),
        |_, _, _| {
            log(LogLevel::INFO, "Thank you for using the ID system!".to_string());
            std::process::exit(0);
        }),
        Command::new("list", None,  |_, _, employees| {
            let employee_list = employees.clone().json();
            let mut index = 0;
            for employee in employee_list {
                index += 1;
                let id = employee.department.clone()+ &employee.role + &employee.id; 
                log(
                    LogLevel::INFO,
                    format!("{}. {} {}, ID: {}", index, employee.first_name, employee.last_name, id)
                );
            }
        })
    ];
}