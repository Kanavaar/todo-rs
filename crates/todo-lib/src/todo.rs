use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

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

    fn add(title: String) {
        if title.len() < 1 {
            println!(
                "{error}{msg}",
                error = "Error: ".red().bold(),
                msg = "No title provided, please provide a title".italic()
            );

            return;
        }

        let mut todos = crate::utils::get_todos().unwrap();
        let todo = Todo {
            title: String::from(&title),
            ..Default::default()
        };

        todos.push(todo);
        crate::utils::save_todos(todos);
        println!(
            "{msg}{title}",
            msg = "Added Todo: ".green().bold(),
            title = title.as_str().italic()
        );
    }

    fn list() {
        let todos = crate::utils::get_todos().unwrap();

        if todos.len() < 1 {
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
                if todo.done {
                    "Completed ?"
                } else {
                    "No ?"
                }
            );
        }
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
