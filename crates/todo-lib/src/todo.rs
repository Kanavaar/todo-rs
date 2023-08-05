use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    created_at: String,
    modified_at: String,
    done: bool,
    title: String,
    id: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataFile {
    data: Vec<Todo>,
}

impl Todo {
    fn timestamp() -> String {
        chrono::Local::now().format("%d.%m, %H:%M").to_string()
    }
}
