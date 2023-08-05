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

    fn id() -> usize {
        use rand::Rng;
        let id: usize = rand::thread_rng().gen_range(1..=10000);
        id + rand::thread_rng().gen_range(1..=10000)
    }
}
