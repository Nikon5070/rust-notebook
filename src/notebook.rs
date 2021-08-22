use serde_json;
use serde_json::json;

use std::fs::File;
use std::io::{BufReader, Error};

use super::config::DB_FILE;
use super::db::JsonFile;
use super::tasks::Tasks;

pub struct Notebook {
    tasks: Tasks,
}

impl Notebook {
    pub fn new() -> Notebook {
        Notebook { tasks: vec![] }
    }

    fn connection_db(&self) -> Result<File, Error> {
        std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&DB_FILE)
    }

    pub fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }
    pub fn add_tasks(&mut self, tasks: &mut Tasks) {
        self.tasks.append(tasks);
    }

    fn print(&self) {
        for (index, value) in self.tasks.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    pub fn read_from_db(&self) -> Result<JsonFile, Error> {
        let f = self.connection_db()?;
        let reader = BufReader::new(f);
        let json_data = serde_json::from_reader(reader)?;
        Ok(json_data)
    }

    pub fn save(&self) -> Result<(), Error> {
        let f = self.connection_db()?;
        let result = json!({
            "tasks": self.tasks
        });
        serde_json::to_writer_pretty(f, &result)?;
        Ok(())
    }
}
