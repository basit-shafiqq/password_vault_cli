fn main() {
    clr();

    let ascii = r#"
  _____         _____ _______          ______  _____  _____   __      __    _    _ _   _______
  |  __ \ /\    / ____/ ____\ \        / / __ \|  __ \|  __ \  \ \    / /\  | |  | | | |__   __|
  | |__) /  \  | (___| (___  \ \  /\  / / |  | | |__) | |  | |  \ \  / /  \ | |  | | |    | |
  |  ___/ /\ \  \___ \\___ \  \ \/  \/ /| |  | |  _  /| |  | |   \ \/ / /\ \| |  | | |    | |
  | |  / ____ \ ____) |___) |  \  /\  / | |__| | | \ \| |__| |    \  / ____ \ |__| | |____| |
  |_| /_/    \_\_____/_____/__  \/ _\/ __\____/|_|  \_\_____/____ _\/_/__  \_\____/|______|_|
                       |  _ \ \   / / |  _ \   /\    / ____|_   _|__   __|
                       | |_) \ \_/ /  | |_) | /  \  | (___   | |    | |
                       |  _ < \   /   |  _ < / /\ \  \___ \  | |    | |
                       | |_) | | |    | |_) / ____ \ ____) |_| |_   | |
                       |____/  |_|    |____/_/    \_\_____/|_____|  |_|
    "#;

    println!("{}", ascii);

    loop {
        println!("Password Manager Menu:");
        println!("\t1. Add Entry");
        println!("\t2. List Entries");
        println!("\t3. Search Entry");
        println!("\t4. Quit");

        let mut choice = String::new();

        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = Service_Info::from_user_input();
                println!("Entry added successfully :)");
                entry.write_to_file();
            }
            "2" => {
                clr();
                match Service_Info::read_passwords() {
                    Ok(services) => {
                        for item in &services {
                            println!(
                                "Service = {}
                                Username = {}
                                Password = {}",
                                item.service, item.username, item.password
                            );
                        }
                    }
                    Err(err) => eprintln!("Application Error: {}", err),
                }
            }
            "3" => {
                clr();
                match Service_Info::read_passwords() {
                    Ok(services) => {
                        let search = Service_Info::prompt("Search: ");
                        for item in &services {
                            if item.service == search {
                                println!(
                                    "Service = {}
                                    Username = {}
                                    Password = {}",
                                    item.service, item.username, item.password
                                );
                            }
                        }
                    }
                    Err(err) => eprintln!("Application Error: {}", err),
                }
            }
            "4" => {
                clr();
                println!("");
                break;
            }
            _ => println!("Invalid Choice!!"),
        }
    }
}

fn clr() {
    print!("{}[2J", 27 as char);
}

use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct Service_Info {
    pub service: String,
    pub username: String,
    pub password: String,
}

impl Service_Info {
    pub fn new(service: String, username: String, password: String) -> Self {
        Service_Info {
            service,
            username,
            password,
        }
    }

    pub fn from_user_input() -> Self {
       
        let service = Service_Info::prompt("Enter service:");

        
        let username = Service_Info::prompt("Enter username:");

       
        let password = Service_Info::prompt("Enter password:");

        Service_Info::new(service, username, password)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize!")
    }

    pub fn write_to_file(&self) {
        let json_output = format!("{}\n", self.to_json());

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("password.json")
            .expect("Failed to open file");

        if let Err(e) = file.write_all(json_output.as_bytes()) {
            eprintln!("Error writing to file: {}", e);
        }
    }

    pub fn prompt(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_string)
    }

    pub fn read_passwords() -> Result<Vec<Service_Info>, io::Error> {
        let file = File::open("password.json");
        if let Ok(file) = file {
            let reader = io::BufReader::new(file);
            let mut services = Vec::new();

            for line in reader.lines() {
                if let Ok(json_string) = line {
                    if let Ok(service_info) = Service_Info::from_json(&json_string) {
                        services.push(service_info);
                    }
                }
            }
            Ok(services)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Password file not found",
            ))
        }
    }
}
