use serde::Deserialize;
use serde_json::json;

use std::fs::File;
use std::io::{BufReader, Error};

const DB_FILE: &'static str = "db.json";

type Tasks = Vec<String>;

#[derive(Deserialize, Debug)]
struct JsonFile {
    tasks: Tasks,
}

struct Notebook {
    tasks: Tasks,
}

impl Notebook {
    fn new() -> Notebook {
        Notebook { tasks: vec![] }
    }

    fn connection_db(&self) -> Result<File, Error> {
        std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&DB_FILE)
    }

    fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }
    fn add_tasks(&mut self, tasks: &mut Tasks) {
        self.tasks.append(tasks);
    }

    fn print(&self) {
        for (index, value) in self.tasks.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    fn read_from_db(&self) -> Result<JsonFile, Error> {
        let f = self.connection_db()?;
        let reader = BufReader::new(f);
        let json_data = serde_json::from_reader(reader)?;
        Ok(json_data)
    }

    fn save(&self) -> Result<(), Error> {
        let f = self.connection_db()?;
        let result = json!({
            "tasks": self.tasks
        });
        serde_json::to_writer_pretty(f, &result)?;
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let task = std::env::args().nth(1).expect("Please specify a task");
    let mut notebook = Notebook::new();
    let mut json_data = notebook.read_from_db()?;

    notebook.add_tasks(&mut json_data.tasks);
    notebook.add_task(task);
    match notebook.save() {
        Ok(_) => println!("todo saved"),
        Err(why) => println!("An error occurred: {}", why),
    }
    Ok(())
}
