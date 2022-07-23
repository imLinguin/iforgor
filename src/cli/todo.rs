use serde::{Deserialize, Serialize};
use std::io::prelude::Write;
use std::io::{stdin, stdout};

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDo {
    name: String,
    description: String,
    done: bool,
}

impl ToDo {
    pub fn cli_new() -> ToDo {
        let mut name = String::new();
        let mut description = String::new();
        loop {
            stdout().write(b"(name) ").unwrap();
            stdout().flush().unwrap();
            stdin()
                .read_line(&mut name)
                .expect("Error getting the name");

            if !name.trim().is_empty() {
                break;
            } else {
                println!("Name cannot be empty!")
            }
        }

        stdout().write(b"(description) ").unwrap();
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut description)
            .expect("Error getting the description");

        ToDo {
            name: String::from(name.trim()),
            description: description.trim().to_string(),
            done: false,
        }
    }
    pub fn change_status(&mut self) {
        self.done = !self.done;
    }
}

fn get_done_character(done: bool) -> char {
    if done {
        '✔'
    } else {
        ' '
    }
}

impl std::fmt::Display for ToDo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", get_done_character(self.done), self.name)
    }
}
