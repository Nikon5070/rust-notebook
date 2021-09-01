mod cli;
mod config;
mod db;
mod notebook;
mod tasks;

use self::cli::Cli;
use self::config::Config;
use crate::notebook::Notebook;
use std::collections::HashMap;
use std::env;
use std::io::Error;

fn init_events() -> HashMap<String, String> {
    let mut book_reviews = HashMap::new();
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    book_reviews
}

fn main() -> Result<(), Error> {
    //arguments
    let args: Vec<String> = env::args().collect();

    //config
    let config = Config::new(&args);

    //app
    let app = Cli::new(config.clone());
    let map = init_events();
    app.run(map);
    //run()
    // expect("Please specify a task");
    let task = config.command.clone();

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
