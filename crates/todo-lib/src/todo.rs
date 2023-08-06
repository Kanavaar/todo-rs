use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    created_at: String,
    modified_at: String,
    done: bool,
    title: String,
    id: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataFile {
    data: Vec<Todo>,
}

impl Todo {
    fn timestamp() -> String {
        chrono::Local::now().format("%d.%m, %H:%M").to_string()
    }

    fn id() -> usize {
        use rand::Rng;
        let id: usize = rand::thread_rng().gen_range(1..=10000);
        id + rand::thread_rng().gen_range(1..=10000)
    }

    pub fn add(title: String) {
        if title.len() == 0 {
            println!(
                "{error}{msg}",
                error = "Error: ".red().bold(),
                msg = "No title provided, please provide a title".italic()
            );

            return;
        }

        let mut todos = crate::todo::Todo::get().unwrap();
        let todo = Todo {
            title: String::from(&title),
            ..Default::default()
        };

        todos.push(todo);
        crate::todo::Todo::save(todos);
        println!(
            "{msg}{title}",
            msg = "Added Todo: ".green().bold(),
            title = title.as_str().italic()
        );
    }

    pub fn list() {
        let todos = crate::todo::Todo::get().unwrap();

        if todos.len() == 0 {
            println!(
                "{error}{msg}",
                error = "Error: ".red().bold(),
                msg = "No Todos Found".italic()
            );
        }

        println!(
            "{0: <5} | {1: <20} | {2: <20} | {3: <20} | {4: <20}",
            "ID", "Title", "Created at", "Updated at", "Done"
        );

        println!();

        for todo in todos {
            println!(
                "{0: <5} | {1: <20} | {2: <20} | {3: <20} | {4: <20}",
                todo.id,
                todo.title,
                todo.created_at,
                todo.modified_at,
                if todo.done { "Completed ?" } else { "No ?" }
            );
        }
    }

    pub fn done(id: String) {
        let mut todos = crate::todo::Todo::get().unwrap();

        let id = match id.parse::<usize>() {
            Ok(id) => id,
            Err(_) => {
                println!(
                    "{error}{msg}",
                    error = "Error: ".red().bold(),
                    msg = "Please provide the id as a positive number".italic()
                );
                return;
            }
        };

        let exists = todos.iter().any(|todo| todo.id == id);
        if !exists {
            println!(
                "{error}{msg}",
                error = "Error: ".red().bold(),
                msg = "Todo not found, please provide a valid id".italic()
            );
            return;
        }

        for todo in &mut todos {
            if todo.id == id {
                todo.done = true;
                todo.modified_at = Todo::timestamp();
            }
        }

        crate::todo::Todo::save(todos);
        println!("{msg}", msg = "Marked todo as done".green().bold());
    }

    pub fn remove(id: String) {
        let mut todos = crate::todo::Todo::get().unwrap();

        let id = match id.parse::<usize>() {
            Ok(id) => id,
            Err(_) => {
                println!(
                    "{error}{msg}",
                    error = "Error: ".red().bold(),
                    msg = "Please provide a positive number".italic()
                );
                return;
            }
        };

        let exists = todos.iter().any(|todo| todo.id == id);
        if !exists {
            println!(
                "{error}{msg}",
                error = "Error: ".red().bold(),
                msg = "Please provide a valid id".italic()
            );
            return;
        }

        todos.retain(|todo| todo.id != id);
        crate::todo::Todo::save(todos);
        println!("{msg}", msg = "Remove Todo".green().bold());
    }

    pub fn get() -> Result<Vec<crate::todo::Todo>, Box<dyn std::error::Error>> {
        let data = std::fs::read_to_string(crate::utils::PROJECT.data_file()).unwrap();
        let todos: crate::todo::DataFile = serde_json::from_str(&data).unwrap();

        Ok(todos.data())
    }

    pub fn save(todos: Vec<crate::todo::Todo>) {
        let data_file = crate::todo::DataFile::from(todos);
        let json = serde_json::to_string(&data_file).unwrap();

        let mut file = std::fs::File::create(crate::utils::PROJECT.data_file()).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}

impl Default for Todo {
    fn default() -> Todo {
        Todo {
            title: "".to_string(),
            modified_at: Todo::timestamp(),
            created_at: Todo::timestamp(),
            done: false,
            id: Todo::id(),
        }
    }
}

impl DataFile {
    // Access Functions
    pub fn data(&self) -> Vec<Todo> {
        self.data.clone()
    }

    pub fn from(data: Vec<Todo>) -> DataFile {
        DataFile { data }
    }
}
