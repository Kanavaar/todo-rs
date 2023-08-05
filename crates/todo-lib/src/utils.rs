use directories;
use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use serde_json::{self, from_str};
use std::io::Write;

struct Files {
    data_dir: std::path::PathBuf,
    data_file: std::ffi::OsString,
}

impl Files {
    fn new(qualifier: &str, org: &str, name: &str) -> Files {
        let mut data_dir = (match directories::ProjectDirs::from(qualifier, org, name) {
            Some(project) => project,
            None => panic!("Could not get ProjectDirs"),
        })
        .data_dir()
        .to_owned();

        let mut data_file = data_dir.as_mut_os_str().to_owned();
        data_file.push("/todos.json");

        Files {
            data_dir,
            data_file,
        }
    }

    fn data_dir(&self) -> &std::path::PathBuf {
        &self.data_dir
    }

    fn data_file(&self) -> &std::ffi::OsString {
        &self.data_file
    }
}

struct Command {
    command: String,
    argument: String,
}

impl Command {
    fn new() -> Command {
        let args = std::env::args().collect::<Vec<String>>();

        let default = "".to_string();
        let command = (match args.get(0) {
            Some(command) => command,
            None => &default,
        })
        .to_string();
        let argument = (match args.get(1) {
            Some(arg) => arg,
            None => &default,
        })
        .to_string();
        Command { command, argument }
    }
}

const PROJECT: Lazy<Files> = Lazy::new(|| Files::new("", "", "rodos"));

pub fn init() {
    if !std::fs::metadata(PROJECT.data_dir()).is_ok() {
        std::fs::create_dir(PROJECT.data_dir()).unwrap();
        println!(
            "{} {}",
            "Created Directory:".green().bold(),
            PROJECT.data_dir().to_str().unwrap().black().italic()
        );
    };

    if !std::fs::metadata(PROJECT.data_file()).is_ok() {
        let mut file = std::fs::File::create(PROJECT.data_file()).unwrap();
        file.write_all(b"\"data\":[]").unwrap();
        println!(
            "{} {}",
            "Created Data File:".green().bold(),
            PROJECT.data_file().to_str().unwrap().black().italic()
        );
    }
}

pub fn get_todos() -> Result<Vec<crate::todo::Todo>, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(PROJECT.data_file()).unwrap();
    let todos: crate::todo::DataFile = from_str(&data)?;

    Ok(todos.data())
}

pub fn save_todos(todos: Vec<crate::todo::Todo>) {
    let data_file = crate::todo::DataFile::from(todos);
    let json = serde_json::to_string(&data_file).unwrap();

    let mut file = std::fs::File::create(PROJECT.data_file()).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
