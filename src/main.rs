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
}

fn main() {
    let task = std::env::args().nth(1).expect("Please specify a task");
    let mut notebook = Notebook::new();
    notebook.add_task(task);

    notebook.print();
}
