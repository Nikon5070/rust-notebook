const DB_TXT: &'static str = "db.txt";

struct Notebook {
    tasks: Vec<String>
}

impl Notebook {
    fn new() -> Notebook {
        Notebook {
            tasks: vec![]
        }
    }

    fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    fn print(&self) {
        for (index, value) in  self.tasks.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    fn save(&self) -> Result<(), std::io::Error> {
        let mut content = String::new();

        for (index, value) in  self.tasks.iter().enumerate() {
            let record = format!("{}:\t{}\n", index, value);
            content.push_str(&record)
        }
        std::fs::write(DB_TXT, content)
    }
}

fn main() {
    let task = std::env::args().nth(1).expect("Please specify a task");
    let mut notebook = Notebook::new();
    notebook.add_task(task);

    match notebook.save() {
        Ok(_) => println!("todo saved"),
        Err(why) => println!("An error occurred: {}", why),
    }
}
