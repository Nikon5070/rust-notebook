mod config;
mod db;
mod notebook;
mod tasks;

use crate::notebook::Notebook;
use std::io::Error;

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
