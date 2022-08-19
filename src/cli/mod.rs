mod todo;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use serde_json;
use std::fs::{create_dir, File};
use std::io::prelude::{Read, Write};
use std::path::{Path, PathBuf};

pub enum Command {
    Unknown,
    Nothing,
    List,
    Exit,
    Create,
    Done(usize),
    Delete(usize),
    Details(usize),
}

pub struct ConfigManager {
    config_path: PathBuf,
}

pub struct Cli {
    config_manager: ConfigManager,
    todos: Vec<todo::ToDo>,
    history_path: PathBuf,
    rl: Editor<()>,
}

impl ConfigManager {
    pub fn new(config_path: PathBuf) -> ConfigManager {
        ConfigManager { config_path }
    }
    fn load_config(&mut self) -> Vec<todo::ToDo> {
        if !self.config_path.exists() {
            create_dir(&self.config_path).expect("Unable to setup config path");
        }
        let file = self.config_path.join("todos.json");

        if file.exists() {
            // Load the file here
            let mut buffer = String::new();
            let mut opened_file = File::open(&file).unwrap();

            opened_file
                .read_to_string(&mut buffer)
                .expect("Error reading todos data from");

            let array: Vec<todo::ToDo> = serde_json::from_str(buffer.as_str()).unwrap();
            array
        } else {
            match File::create(&file) {
                Ok(mut file) => {
                    file.write_all(b"[]").unwrap();
                }
                Err(_) => {
                    panic!("Unable to create the config file");
                }
            }
            vec![]
        }
    }
    fn write_config(&mut self, todos: &Vec<todo::ToDo>) {
        let file = self.config_path.join("todos.json");
        let mut opened_file = File::create(file).unwrap();

        let data: String = serde_json::to_string(todos).expect("Unable to serialize todos");
        opened_file
            .write(data.as_bytes())
            .expect("Error when writing to config");
    }
}

impl Cli {
    pub fn new(config_path: PathBuf) -> Cli {
        let mut instance = Cli {
            config_manager: ConfigManager::new(config_path.clone()),
            history_path: config_path.join(Path::new("history")),
            todos: Vec::new(),
            rl: Editor::<()>::new().unwrap(),
        };

        if instance
            .rl
            .load_history(instance.history_path.as_path())
            .is_err()
        {
            instance
                .rl
                .save_history(instance.history_path.as_path())
                .unwrap();
        }
        instance.todos = instance.config_manager.load_config();

        return instance;
    }
    fn handle_exit(&mut self) {
        self.rl.save_history(self.history_path.as_path()).unwrap();
        self.save();
    }
    fn read_stdin(&mut self, text: &str) -> String {
        match self.rl.readline(text) {
            Ok(value) => value,
            Err(ReadlineError::Interrupted) => {
                self.handle_exit();
                std::process::exit(0);
            }
            Err(ReadlineError::Eof) => String::new(),
            Err(_) => String::new(),
        }
    }

    fn get_command(&mut self) -> Command {
        let command: String = self.read_stdin("iforgor 💀> ");

        match command.trim() {
            "exit" => Command::Exit,
            "list" | "ls" => {
                self.rl.add_history_entry(&command);
                Command::List
            }
            "clear_history" => {
                self.rl.clear_history();
                self.rl.save_history(self.history_path.as_path()).unwrap();
                Command::Nothing
            }
            "create" | "add" => {
                self.rl.add_history_entry(&command);
                Command::Create
            }
            _ if command.starts_with("show") || command.starts_with("details") => {
                self.rl.add_history_entry(&command);
                let id: String;
                let arguments = command.trim().split(' ');
                if arguments.clone().count() == 1 {
                    id = self.read_stdin("(id) ")
                } else {
                    id = arguments.last().unwrap().to_string();
                }
                let id_usize: usize = match id.trim().parse() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Unable to parse the id");
                        0
                    }
                };
                Command::Details(id_usize)
            }
            _ if command.starts_with("delete") || command.starts_with("remove") => {
                self.rl.add_history_entry(&command);
                let id: String;
                let arguments = command.trim().split(' ');
                if arguments.clone().count() == 1 {
                    id = self.read_stdin("(id) ")
                } else {
                    id = arguments.last().unwrap().to_string();
                }

                let id_usize: usize = match id.trim().parse() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Unable to parse the id");
                        0
                    }
                };
                Command::Delete(id_usize)
            }
            _ if command.starts_with("done") => {
                self.rl.add_history_entry(&command);
                let id: String;
                let arguments = command.trim().split(' ');
                if arguments.clone().count() == 1 {
                    id = self.read_stdin("(id) ")
                } else {
                    id = arguments.last().unwrap().to_string();
                }

                let id_usize: usize = match id.trim().parse() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Unable to parse the id");
                        0
                    }
                };
                Command::Done(id_usize)
            }
            _ => {
                self.rl.add_history_entry(&command);
                Command::Unknown
            }
        }
    }
    fn save(&mut self) {
        self.config_manager.write_config(&self.todos);
    }
    pub fn run(&mut self) {
        loop {
            let command = self.get_command();
            match command {
                Command::Exit => {
                    self.handle_exit();
                    std::process::exit(0)
                }
                Command::List => {
                    let mut index = 1;
                    for todo in self.todos.iter() {
                        println!("{}. {}", index, todo);
                        index += 1;
                    }
                }
                Command::Delete(id) => {
                    if id == 0 {
                        println!("Such todo doesn't exist"); // id is usize so number can't be negative
                        continue;
                    }
                    let id = id - 1;

                    if id >= self.todos.len() {
                        println!("Such todo doesn't exist");
                        continue;
                    }

                    self.todos.remove(id);
                }
                Command::Done(id) => {
                    if id == 0 {
                        println!("Such todo doesn't exist"); // id is usize so number can't be negative
                        continue;
                    }
                    let id = id - 1;

                    if id >= self.todos.len() {
                        println!("Such todo doesn't exist");
                        continue;
                    }

                    let todo = self.todos.get_mut(id).unwrap();
                    todo.change_status();
                }
                Command::Create => {
                    let new_todo = todo::ToDo::cli_new();
                    self.todos.push(new_todo);
                }
                Command::Details(id) => {
                    if id == 0 {
                        println!("Such todo doesn't exist"); // id is usize so number can't be negative
                        continue;
                    }
                    let id = id - 1;

                    if id >= self.todos.len() {
                        println!("Such todo doesn't exist");
                        continue;
                    }

                    let todo = self.todos.get_mut(id).unwrap();
                    todo.print_details();
                }
                Command::Nothing => continue,
                Command::Unknown => {
                    println!("Unknown")
                }
            }
        }
    }
}
